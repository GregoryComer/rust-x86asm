use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 58, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Word)
}

#[test]
fn cmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(ECX, 2027135360, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledDisplaced(EDX, Eight, 1694840547, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Dword)
}

#[test]
fn cmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1262403651, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Four, 1157096900, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Qword)
}

#[test]
fn cmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(DI, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Word)
}

#[test]
fn cmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(ECX, 1527102838, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(EDI, 1180810120, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Dword)
}

#[test]
fn cmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(RSI, RCX, Eight, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Qword)
}

#[test]
fn cmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 130, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(DI, 96, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Word)
}

#[test]
fn cmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 1534647410, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Dword)
}

#[test]
fn cmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RDX, Eight, 589211528, Some(OperandSize::Dword), None)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Qword)
}

#[test]
fn cmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(RSI, Two, 811000742, Some(OperandSize::Qword), None)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 167], OperandSize::Qword)
}

