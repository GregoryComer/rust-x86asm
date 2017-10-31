use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn cmovnp_1() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 242], OperandSize::Word)
}

#[test]
fn cmovnp_2() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SI)), operand2: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 183, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 176, 183, 0], OperandSize::Word)
}

#[test]
fn cmovnp_3() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(CX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 207], OperandSize::Dword)
}

#[test]
fn cmovnp_4() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(BP)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 494000463, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 172, 248, 79, 217, 113, 29], OperandSize::Dword)
}

#[test]
fn cmovnp_5() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 204], OperandSize::Qword)
}

#[test]
fn cmovnp_6() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(SP)), operand2: Some(IndirectScaledDisplaced(RCX, Four, 592886493, Some(OperandSize::Word), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 36, 141, 221, 186, 86, 35], OperandSize::Qword)
}

#[test]
fn cmovnp_7() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 251], OperandSize::Word)
}

#[test]
fn cmovnp_8() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(IndirectDisplaced(BX, 2027, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 75, 183, 235, 7], OperandSize::Word)
}

#[test]
fn cmovnp_9() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 210], OperandSize::Dword)
}

#[test]
fn cmovnp_10() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(EBX)), operand2: Some(IndirectDisplaced(EDX, 584564717, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 154, 237, 191, 215, 34], OperandSize::Dword)
}

#[test]
fn cmovnp_11() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 243], OperandSize::Qword)
}

#[test]
fn cmovnp_12() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(ECX)), operand2: Some(Indirect(RAX, Some(OperandSize::Dword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 75, 8], OperandSize::Qword)
}

#[test]
fn cmovnp_13() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 228], OperandSize::Qword)
}

#[test]
fn cmovnp_14() {
    run_test(&Instruction { mnemonic: Mnemonic::CMOVNP, operand1: Some(Direct(RDX)), operand2: Some(IndirectScaledDisplaced(RDI, Four, 2093271138, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 75, 20, 189, 98, 200, 196, 124], OperandSize::Qword)
}

