use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovng_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 214], OperandSize::Word)
}

#[test]
fn cmovng_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DX)), operand2: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 18], OperandSize::Word)
}

#[test]
fn cmovng_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(DX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 214], OperandSize::Dword)
}

#[test]
fn cmovng_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 33], OperandSize::Dword)
}

#[test]
fn cmovng_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 229], OperandSize::Qword)
}

#[test]
fn cmovng_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexed(RBX, RSI, Two, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 44, 115], OperandSize::Qword)
}

#[test]
fn cmovng_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 228], OperandSize::Word)
}

#[test]
fn cmovng_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 201, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 78, 162, 201, 0], OperandSize::Word)
}

#[test]
fn cmovng_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 225], OperandSize::Dword)
}

#[test]
fn cmovng_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ECX)), operand2: Some(IndirectScaledDisplaced(EAX, Eight, 1398730736, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 12, 197, 240, 239, 94, 83], OperandSize::Dword)
}

#[test]
fn cmovng_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 239], OperandSize::Qword)
}

#[test]
fn cmovng_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(ESP)), operand2: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 78, 36, 184], OperandSize::Qword)
}

#[test]
fn cmovng_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 250], OperandSize::Qword)
}

#[test]
fn cmovng_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNG, operand1: Some(Direct(RSP)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 977029431, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 78, 36, 213, 55, 73, 60, 58], OperandSize::Qword)
}

