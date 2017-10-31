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
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 31], OperandSize::Word)
}

#[test]
fn bts_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 207], OperandSize::Dword)
}

#[test]
fn bts_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EDI, Two, 1626604679, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 148, 120, 135, 4, 244, 96], OperandSize::Dword)
}

#[test]
fn bts_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DI)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 223], OperandSize::Qword)
}

#[test]
fn bts_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(RDI, 1541424505, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 151, 121, 69, 224, 91], OperandSize::Qword)
}

#[test]
fn bts_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 247], OperandSize::Word)
}

#[test]
fn bts_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 37, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 116, 37], OperandSize::Word)
}

#[test]
fn bts_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 227], OperandSize::Dword)
}

#[test]
fn bts_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(ESI, 1264733795, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 142, 99, 78, 98, 75], OperandSize::Dword)
}

#[test]
fn bts_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 237], OperandSize::Qword)
}

#[test]
fn bts_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RBX, RBX, Two, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 60, 91], OperandSize::Qword)
}

#[test]
fn bts_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 235], OperandSize::Qword)
}

#[test]
fn bts_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 38], OperandSize::Qword)
}

#[test]
fn bts_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(SP)), operand2: Some(Literal8(118)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 236, 118], OperandSize::Word)
}

#[test]
fn bts_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(BP, 246, Some(OperandSize::Word), None)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 174, 246, 0, 70], OperandSize::Word)
}

#[test]
fn bts_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 34], OperandSize::Dword)
}

#[test]
fn bts_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(ECX, Four, 1619450912, Some(OperandSize::Word), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 141, 32, 220, 134, 96, 7], OperandSize::Dword)
}

#[test]
fn bts_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Literal8(88)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 88], OperandSize::Qword)
}

#[test]
fn bts_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(101)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 121, 101], OperandSize::Qword)
}

#[test]
fn bts_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ECX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 233, 123], OperandSize::Word)
}

#[test]
fn bts_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 47, 46], OperandSize::Word)
}

#[test]
fn bts_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDI)), operand2: Some(Literal8(75)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 239, 75], OperandSize::Dword)
}

#[test]
fn bts_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(ECX, EDI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 44, 249, 113], OperandSize::Dword)
}

#[test]
fn bts_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 238, 103], OperandSize::Qword)
}

#[test]
fn bts_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RDI, RDI, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(112)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 44, 191, 112], OperandSize::Qword)
}

#[test]
fn bts_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RBP)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 237, 90], OperandSize::Qword)
}

#[test]
fn bts_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(RAX, 279446239, Some(OperandSize::Qword), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 168, 223, 2, 168, 16, 38], OperandSize::Qword)
}

