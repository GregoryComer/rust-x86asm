use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: Some(Literal8(35)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 206, 35], OperandSize::Word)
}

#[test]
fn shrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(SI, 28208, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 172, 48, 110, 106], OperandSize::Word)
}

#[test]
fn shrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: Some(Literal8(24)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 213, 24], OperandSize::Dword)
}

#[test]
fn shrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(ESI, Four, 2040613922, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 20, 181, 34, 76, 161, 121, 12], OperandSize::Dword)
}

#[test]
fn shrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 234, 57], OperandSize::Qword)
}

#[test]
fn shrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(RDX, RCX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Literal8(77)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 60, 138, 77], OperandSize::Qword)
}

#[test]
fn shrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 202, 112], OperandSize::Word)
}

#[test]
fn shrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 252, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 154, 252, 0, 69], OperandSize::Word)
}

#[test]
fn shrd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 243, 98], OperandSize::Dword)
}

#[test]
fn shrd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 62, 96], OperandSize::Dword)
}

#[test]
fn shrd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 236, 104], OperandSize::Qword)
}

#[test]
fn shrd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RBX, 309707305, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 163, 41, 194, 117, 18, 29], OperandSize::Qword)
}

#[test]
fn shrd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 247, 0], OperandSize::Qword)
}

#[test]
fn shrd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RCX, 1874358576, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 161, 48, 113, 184, 111, 57], OperandSize::Qword)
}

#[test]
fn shrd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 209], OperandSize::Word)
}

#[test]
fn shrd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 19], OperandSize::Word)
}

#[test]
fn shrd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 234], OperandSize::Dword)
}

#[test]
fn shrd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(ECX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 49], OperandSize::Dword)
}

#[test]
fn shrd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 251], OperandSize::Qword)
}

#[test]
fn shrd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(RDI, Four, 101508656, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 12, 189, 48, 230, 12, 6], OperandSize::Qword)
}

#[test]
fn shrd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 242], OperandSize::Word)
}

#[test]
fn shrd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 22, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 82, 22], OperandSize::Word)
}

#[test]
fn shrd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 239], OperandSize::Dword)
}

#[test]
fn shrd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1606910244, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 148, 79, 36, 129, 199, 95], OperandSize::Dword)
}

#[test]
fn shrd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 237], OperandSize::Qword)
}

#[test]
fn shrd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 18], OperandSize::Qword)
}

#[test]
fn shrd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 230], OperandSize::Qword)
}

#[test]
fn shrd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1319083617, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 180, 186, 97, 158, 159, 78], OperandSize::Qword)
}

