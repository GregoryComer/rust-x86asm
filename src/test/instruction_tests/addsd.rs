use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn addsd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 236], OperandSize::Dword)
}

fn addsd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM5)), operand2: Some(IndirectDisplaced(EDI, 461669257, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 175, 137, 131, 132, 27], OperandSize::Dword)
}

fn addsd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 215], OperandSize::Qword)
}

fn addsd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::ADDSD, operand1: Some(Direct(XMM2)), operand2: Some(IndirectDisplaced(RBX, 2072249513, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[242, 15, 88, 147, 169, 4, 132, 123], OperandSize::Qword)
}

