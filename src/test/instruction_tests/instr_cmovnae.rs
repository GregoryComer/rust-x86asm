use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnae_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 230], OperandSize::Word)
}

#[test]
fn cmovnae_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 25768, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 163, 168, 100], OperandSize::Word)
}

#[test]
fn cmovnae_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 207], OperandSize::Dword)
}

#[test]
fn cmovnae_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(CX)), operand2: Some(IndirectDisplaced(EAX, 175631897, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 136, 25, 238, 119, 10], OperandSize::Dword)
}

#[test]
fn cmovnae_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(BP)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 233], OperandSize::Qword)
}

#[test]
fn cmovnae_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(SP)), operand2: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 35], OperandSize::Qword)
}

#[test]
fn cmovnae_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 207], OperandSize::Word)
}

#[test]
fn cmovnae_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDI)), operand2: Some(IndirectDisplaced(BP, 252, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 66, 190, 252, 0], OperandSize::Word)
}

#[test]
fn cmovnae_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 211], OperandSize::Dword)
}

#[test]
fn cmovnae_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(ESI)), operand2: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 48], OperandSize::Dword)
}

#[test]
fn cmovnae_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 214], OperandSize::Qword)
}

#[test]
fn cmovnae_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(EBX)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 354433550, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 66, 156, 209, 14, 58, 32, 21], OperandSize::Qword)
}

#[test]
fn cmovnae_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RDI)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 249], OperandSize::Qword)
}

#[test]
fn cmovnae_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNAE, operand1: Some(Direct(RBX)), operand2: Some(IndirectScaledDisplaced(RSI, Eight, 1797168376, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 66, 28, 245, 248, 156, 30, 107], OperandSize::Qword)
}

