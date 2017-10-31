use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn lss_1() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(DI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 1144, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 185, 120, 4], OperandSize::Word)
}

#[test]
fn lss_2() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(BX)), operand2: Some(IndirectDisplaced(ECX, 163576788, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 153, 212, 251, 191, 9], OperandSize::Dword)
}

#[test]
fn lss_3() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1993988580, Some(OperandSize::Far16), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 178, 52, 245, 228, 217, 217, 118], OperandSize::Qword)
}

#[test]
fn lss_4() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(ESP)), operand2: Some(Indirect(EDX, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 34], OperandSize::Dword)
}

#[test]
fn lss_5() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Eight, Some(OperandSize::Far32), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 178, 12, 243], OperandSize::Qword)
}

#[test]
fn lss_6() {
    run_test(&Instruction { mnemonic: Mnemonic::LSS, operand1: Some(Direct(RDI)), operand2: Some(IndirectDisplaced(RCX, 1632670667, Some(OperandSize::Far64), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 178, 185, 203, 147, 80, 97], OperandSize::Qword)
}

