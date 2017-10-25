use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn subpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 212], OperandSize::Dword)
}

fn subpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexed(EDI, EDX, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 4, 215], OperandSize::Dword)
}

fn subpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 244], OperandSize::Qword)
}

fn subpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SUBPD, operand1: Some(Direct(XMM7)), operand2: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 92, 56], OperandSize::Qword)
}

