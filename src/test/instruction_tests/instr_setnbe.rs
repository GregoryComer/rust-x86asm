use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn setnbe_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Word)
}

#[test]
fn setnbe_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Indirect(SI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4], OperandSize::Word)
}

#[test]
fn setnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Dword)
}

#[test]
fn setnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Indirect(ESI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 6], OperandSize::Dword)
}

#[test]
fn setnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn setnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(RCX, RBX, Eight, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 217], OperandSize::Qword)
}

#[test]
fn setnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn setnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 150], OperandSize::Qword)
}

