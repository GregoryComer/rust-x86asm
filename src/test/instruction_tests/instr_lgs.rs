use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lgs_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(DI)), operand2: Some(IndirectDisplaced(BP, 2408, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 190, 104, 9], OperandSize::Word)
}

#[test]
fn lgs_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(BX)), operand2: Some(IndirectScaledIndexed(ECX, EDX, Two, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 28, 81], OperandSize::Dword)
}

#[test]
fn lgs_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(SP)), operand2: Some(IndirectDisplaced(RBX, 134739760, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 181, 163, 48, 247, 7, 8], OperandSize::Qword)
}

#[test]
fn lgs_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EDX, Two, 1729601014, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 12, 85, 246, 157, 23, 103], OperandSize::Dword)
}

#[test]
fn lgs_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(ESI)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Eight, 1481230998, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 181, 180, 195, 150, 202, 73, 88], OperandSize::Qword)
}

#[test]
fn lgs_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LGS, operand1: Some(Direct(RBP)), operand2: Some(Indirect(RSI, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 181, 46], OperandSize::Qword)
}

