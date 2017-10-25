use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vsubsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 211, 92, 192], OperandSize::Dword)
}

fn vsubsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 639613897, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 137, 201, 187, 31, 38], OperandSize::Dword)
}

fn vsubsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 235, 92, 204], OperandSize::Qword)
}

fn vsubsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RDI, Four, 1854931258, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 243, 92, 12, 189, 58, 1, 144, 110], OperandSize::Qword)
}

fn vsubsd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 199, 251, 92, 217], OperandSize::Dword)
}

fn vsubsd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 223, 137, 92, 55], OperandSize::Dword)
}

fn vsubsd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM22)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 33, 231, 217, 92, 230], OperandSize::Qword)
}

fn vsubsd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSUBSD, operand1: Some(Direct(XMM13)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RBX, 1978696906, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 255, 140, 92, 171, 202, 132, 240, 117], OperandSize::Qword)
}

