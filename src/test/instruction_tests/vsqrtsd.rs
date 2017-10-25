use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsqrtsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 81, 209], OperandSize::Dword)
}

fn vsqrtsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Two, 2030237153, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 164, 94, 225, 245, 2, 121], OperandSize::Dword)
}

fn vsqrtsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 195, 81, 204], OperandSize::Qword)
}

fn vsqrtsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 81, 9], OperandSize::Qword)
}

fn vsqrtsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 223, 190, 81, 215], OperandSize::Dword)
}

fn vsqrtsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, ESI, Two, 1574268159, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 239, 140, 81, 156, 118, 255, 108, 213, 93], OperandSize::Dword)
}

fn vsqrtsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM28)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 33, 159, 210, 81, 229], OperandSize::Qword)
}

fn vsqrtsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSQRTSD, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 183, 137, 81, 44, 143], OperandSize::Qword)
}

