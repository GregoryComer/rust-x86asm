use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vdivsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 94, 223], OperandSize::Dword)
}

fn vdivsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 253916724, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 94, 132, 177, 52, 118, 34, 15], OperandSize::Dword)
}

fn vdivsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 94, 202], OperandSize::Qword)
}

fn vdivsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RAX, 1752390363, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 94, 160, 219, 90, 115, 104], OperandSize::Qword)
}

fn vdivsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 199, 251, 94, 226], OperandSize::Dword)
}

fn vdivsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 247, 142, 94, 24], OperandSize::Dword)
}

fn vdivsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 81, 215, 145, 94, 203], OperandSize::Qword)
}

fn vdivsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VDIVSD, operand1: Some(Direct(XMM16)), operand2: Some(Direct(XMM25)), operand3: Some(IndirectDisplaced(RDI, 1050974502, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 183, 131, 94, 135, 38, 153, 164, 62], OperandSize::Qword)
}

