use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DX)), operand2: Some(Direct(DI)), operand3: Some(Literal8(33)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 250, 33], OperandSize::Word)
}

#[test]
fn shrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 48, 115], OperandSize::Word)
}

#[test]
fn shrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: Some(Literal8(100)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 230, 100], OperandSize::Dword)
}

#[test]
fn shrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(ECX, 54007574, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 145, 22, 23, 56, 3, 25], OperandSize::Dword)
}

#[test]
fn shrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DI)), operand2: Some(Direct(CX)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 207, 69], OperandSize::Qword)
}

#[test]
fn shrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(91)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 54, 91], OperandSize::Qword)
}

#[test]
fn shrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 255, 36], OperandSize::Word)
}

#[test]
fn shrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(BP, 196, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: Some(Literal8(66)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 142, 196, 0, 66], OperandSize::Word)
}

#[test]
fn shrd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(27)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 209, 27], OperandSize::Dword)
}

#[test]
fn shrd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Two, 576874892, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(21)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 188, 112, 140, 105, 98, 34, 21], OperandSize::Dword)
}

#[test]
fn shrd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 239, 43], OperandSize::Qword)
}

#[test]
fn shrd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 55, 40], OperandSize::Qword)
}

#[test]
fn shrd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RBP)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(112)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 205, 112], OperandSize::Qword)
}

#[test]
fn shrd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RDX, 516266521, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Literal8(122)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 146, 25, 154, 197, 30, 122], OperandSize::Qword)
}

#[test]
fn shrd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 244], OperandSize::Word)
}

#[test]
fn shrd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 27440, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 177, 48, 107], OperandSize::Word)
}

#[test]
fn shrd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 225], OperandSize::Dword)
}

#[test]
fn shrd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(ESI, 663955499, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 150, 43, 40, 147, 39], OperandSize::Dword)
}

#[test]
fn shrd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 238], OperandSize::Qword)
}

#[test]
fn shrd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Four, 1456529221, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 156, 186, 69, 223, 208, 86], OperandSize::Qword)
}

#[test]
fn shrd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 203], OperandSize::Word)
}

#[test]
fn shrd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(SI, 4857, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 156, 249, 18], OperandSize::Word)
}

#[test]
fn shrd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 221], OperandSize::Dword)
}

#[test]
fn shrd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(EBX, 1726888789, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 155, 85, 59, 238, 102], OperandSize::Dword)
}

#[test]
fn shrd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 246], OperandSize::Qword)
}

#[test]
fn shrd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(RAX, 409113090, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 184, 2, 146, 98, 24], OperandSize::Qword)
}

#[test]
fn shrd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 228], OperandSize::Qword)
}

#[test]
fn shrd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 22], OperandSize::Qword)
}

