use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 229], OperandSize::Word)
}

#[test]
fn cmovae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(BX)), operand2: Some(Indirect(DI, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 29], OperandSize::Word)
}

#[test]
fn cmovae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(BP)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 235], OperandSize::Dword)
}

#[test]
fn cmovae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(SI)), operand2: Some(IndirectDisplaced(EDI, 1980609669, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 183, 133, 180, 13, 118], OperandSize::Dword)
}

#[test]
fn cmovae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 207], OperandSize::Qword)
}

#[test]
fn cmovae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 1634162530, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 172, 203, 98, 87, 103, 97], OperandSize::Qword)
}

#[test]
fn cmovae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 235], OperandSize::Word)
}

#[test]
fn cmovae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBP)), operand2: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 67, 43], OperandSize::Word)
}

#[test]
fn cmovae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 233], OperandSize::Dword)
}

#[test]
fn cmovae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDI)), operand2: Some(IndirectScaledDisplaced(EDI, Eight, 2070506103, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 60, 253, 119, 106, 105, 123], OperandSize::Dword)
}

#[test]
fn cmovae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 201], OperandSize::Qword)
}

#[test]
fn cmovae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(EDX)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 67, 20, 254], OperandSize::Qword)
}

#[test]
fn cmovae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RCX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 203], OperandSize::Qword)
}

#[test]
fn cmovae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVAE, operand1: Some(Direct(RSP)), operand2: Some(IndirectDisplaced(RAX, 2116633855, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 67, 160, 255, 68, 41, 126], OperandSize::Qword)
}

