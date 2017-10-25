use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn movhpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM3)), operand2: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 26], OperandSize::Dword)
}

fn movhpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectDisplaced(RCX, 809674244, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 22, 129, 4, 166, 66, 48], OperandSize::Qword)
}

fn movhpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectScaledDisplaced(EDX, Four, 405290505, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 60, 149, 9, 62, 40, 24], OperandSize::Dword)
}

fn movhpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::MOVHPD, operand1: Some(IndirectDisplaced(RBX, 1569260861, Some(OperandSize::Qword), None)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 23, 147, 61, 5, 137, 93], OperandSize::Qword)
}

