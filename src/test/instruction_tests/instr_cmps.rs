use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 7541, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(BP, 31488, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Word)
}

#[test]
fn cmps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Four, 661557956, Some(OperandSize::Byte), None)), operand2: Some(IndirectScaledIndexed(EAX, EAX, Eight, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Dword)
}

#[test]
fn cmps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(RDX, 1138108798, Some(OperandSize::Byte), None)), operand2: Some(IndirectDisplaced(RCX, 1004683037, Some(OperandSize::Byte), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[166], OperandSize::Qword)
}

#[test]
fn cmps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectDisplaced(SI, 16, Some(OperandSize::Word), None)), operand2: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Word)
}

#[test]
fn cmps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 339794551, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(EDI, 567186782, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Dword)
}

#[test]
fn cmps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(IndirectDisplaced(RBX, 2002015068, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Qword)
}

#[test]
fn cmps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 167], OperandSize::Word)
}

#[test]
fn cmps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(ECX, Four, 2092912304, Some(OperandSize::Dword), None)), operand2: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Dword)
}

#[test]
fn cmps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(IndirectScaledDisplaced(RDI, Four, 319003707, Some(OperandSize::Dword), None)), operand2: Some(IndirectDisplaced(RSI, 14037176, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[167], OperandSize::Qword)
}

#[test]
fn cmps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMPS, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(IndirectDisplaced(RSI, 1004196963, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 167], OperandSize::Qword)
}

