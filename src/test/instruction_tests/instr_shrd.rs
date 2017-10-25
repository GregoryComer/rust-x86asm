use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 234, 73], OperandSize::Word)
}

#[test]
fn shrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(BP, 30426, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 174, 218, 118, 28], OperandSize::Word)
}

#[test]
fn shrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SP)), operand2: Some(Direct(CX)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 204, 8], OperandSize::Dword)
}

#[test]
fn shrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Two, 638192808, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 164, 121, 168, 12, 10, 38, 27], OperandSize::Dword)
}

#[test]
fn shrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: Some(Literal8(19)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 235, 19], OperandSize::Qword)
}

#[test]
fn shrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RCX, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(38)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 49, 38], OperandSize::Qword)
}

#[test]
fn shrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 241, 25], OperandSize::Word)
}

#[test]
fn shrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 21580, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(113)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 146, 76, 84, 113], OperandSize::Word)
}

#[test]
fn shrd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 234, 83], OperandSize::Dword)
}

#[test]
fn shrd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(EBX, Two, 672101248, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 52, 93, 128, 115, 15, 40, 92], OperandSize::Dword)
}

#[test]
fn shrd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(55)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 230, 55], OperandSize::Qword)
}

#[test]
fn shrd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Four, 1974843738, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 172, 154, 90, 185, 181, 117, 46], OperandSize::Qword)
}

#[test]
fn shrd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Literal8(90)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 230, 90], OperandSize::Qword)
}

#[test]
fn shrd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RAX, 1049948938, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 176, 10, 243, 148, 62, 57], OperandSize::Qword)
}

#[test]
fn shrd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(CX)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 233], OperandSize::Word)
}

#[test]
fn shrd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 4219, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 144, 123, 16], OperandSize::Word)
}

#[test]
fn shrd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 227], OperandSize::Dword)
}

#[test]
fn shrd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(EBX, EBX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 20, 91], OperandSize::Dword)
}

#[test]
fn shrd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 213], OperandSize::Qword)
}

#[test]
fn shrd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 559503603, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 156, 65, 243, 88, 89, 33], OperandSize::Qword)
}

#[test]
fn shrd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 217], OperandSize::Word)
}

#[test]
fn shrd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(BX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 23], OperandSize::Word)
}

#[test]
fn shrd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 218], OperandSize::Dword)
}

#[test]
fn shrd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 15], OperandSize::Dword)
}

#[test]
fn shrd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 226], OperandSize::Qword)
}

#[test]
fn shrd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1896693032, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 20, 69, 40, 61, 13, 113], OperandSize::Qword)
}

#[test]
fn shrd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RBX)), operand2: Some(Direct(RCX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 203], OperandSize::Qword)
}

#[test]
fn shrd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 20, 67], OperandSize::Qword)
}

