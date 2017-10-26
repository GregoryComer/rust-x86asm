use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn fistp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 31], OperandSize::Word)
}

#[test]
fn fistp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(ESI, EBX, Eight, 556498377, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 156, 222, 201, 125, 43, 33], OperandSize::Dword)
}

#[test]
fn fistp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[219, 26], OperandSize::Qword)
}

#[test]
fn fistp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 20, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 90, 20], OperandSize::Word)
}

#[test]
fn fistp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledDisplaced(EAX, Two, 510629163, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 28, 69, 43, 149, 111, 30], OperandSize::Dword)
}

#[test]
fn fistp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 24], OperandSize::Qword)
}

#[test]
fn fistp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 4575, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 187, 223, 17], OperandSize::Word)
}

#[test]
fn fistp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectScaledDisplaced(EDX, Two, 315565102, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 60, 85, 46, 36, 207, 18], OperandSize::Dword)
}

#[test]
fn fistp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::FISTP, operand1: Some(IndirectDisplaced(RDX, 1815795331, Some(OperandSize::Qword), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[223, 186, 131, 214, 58, 108], OperandSize::Qword)
}

