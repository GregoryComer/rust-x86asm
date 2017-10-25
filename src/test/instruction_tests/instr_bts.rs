use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bts_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 203], OperandSize::Word)
}

#[test]
fn bts_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(BX, 247, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 183, 247, 0], OperandSize::Word)
}

#[test]
fn bts_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(SP)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 236], OperandSize::Dword)
}

#[test]
fn bts_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Eight, 1967312959, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 188, 248, 63, 208, 66, 117], OperandSize::Dword)
}

#[test]
fn bts_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 211], OperandSize::Qword)
}

#[test]
fn bts_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 108023302, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 188, 193, 6, 78, 112, 6], OperandSize::Qword)
}

#[test]
fn bts_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 210], OperandSize::Word)
}

#[test]
fn bts_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 33, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 108, 33], OperandSize::Word)
}

#[test]
fn bts_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 234], OperandSize::Dword)
}

#[test]
fn bts_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(ECX, 941909690, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 145, 186, 102, 36, 56], OperandSize::Dword)
}

#[test]
fn bts_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 220], OperandSize::Qword)
}

#[test]
fn bts_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 51], OperandSize::Qword)
}

#[test]
fn bts_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RCX)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 201], OperandSize::Qword)
}

#[test]
fn bts_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RDX, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 44, 186], OperandSize::Qword)
}

#[test]
fn bts_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DI)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 239, 117], OperandSize::Word)
}

#[test]
fn bts_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(BP, 63, Some(OperandSize::Word), None)), operand2: Some(Literal8(71)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 110, 63, 71], OperandSize::Word)
}

#[test]
fn bts_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DX)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 234, 121], OperandSize::Dword)
}

#[test]
fn bts_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 42, 19], OperandSize::Dword)
}

#[test]
fn bts_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DI)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 239, 125], OperandSize::Qword)
}

#[test]
fn bts_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1626083603, Some(OperandSize::Word), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 149, 19, 17, 236, 96, 78], OperandSize::Qword)
}

#[test]
fn bts_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Literal8(8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 239, 8], OperandSize::Word)
}

#[test]
fn bts_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 7991, Some(OperandSize::Dword), None)), operand2: Some(Literal8(85)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 168, 55, 31, 85], OperandSize::Word)
}

#[test]
fn bts_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDX)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 234, 37], OperandSize::Dword)
}

#[test]
fn bts_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 236518028, Some(OperandSize::Dword), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 44, 245, 140, 250, 24, 14, 55], OperandSize::Dword)
}

#[test]
fn bts_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 239, 103], OperandSize::Qword)
}

#[test]
fn bts_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 61263768, Some(OperandSize::Dword), None)), operand2: Some(Literal8(78)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 172, 121, 152, 207, 166, 3, 78], OperandSize::Qword)
}

#[test]
fn bts_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RSP)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 236, 37], OperandSize::Qword)
}

#[test]
fn bts_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RCX, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 44, 65, 90], OperandSize::Qword)
}

