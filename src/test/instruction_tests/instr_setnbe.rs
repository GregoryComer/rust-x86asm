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
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 11465, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 131, 201, 44], OperandSize::Word)
}

#[test]
fn setnbe_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Dword)
}

#[test]
fn setnbe_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Indirect(EDI, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 7], OperandSize::Dword)
}

#[test]
fn setnbe_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(CL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 193], OperandSize::Qword)
}

#[test]
fn setnbe_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledDisplaced(RDI, Four, 1924434470, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 189, 38, 138, 180, 114], OperandSize::Qword)
}

#[test]
fn setnbe_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(Direct(BL)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 195], OperandSize::Qword)
}

#[test]
fn setnbe_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SETNBE, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 2115429564, Some(OperandSize::Byte), None)), operand2: None, operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 151, 4, 253, 188, 228, 22, 126], OperandSize::Qword)
}

