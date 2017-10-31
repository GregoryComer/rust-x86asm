use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shrd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: Some(Literal8(51)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 215, 51], OperandSize::Word)
}

#[test]
fn shrd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 21, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(68)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 83, 21, 68], OperandSize::Word)
}

#[test]
fn shrd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BP)), operand2: Some(Direct(DI)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 253, 114], OperandSize::Dword)
}

#[test]
fn shrd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Eight, 429599595, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: Some(Literal8(114)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 140, 240, 107, 43, 155, 25, 114], OperandSize::Dword)
}

#[test]
fn shrd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SI)), operand2: Some(Direct(DX)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 214, 29], OperandSize::Qword)
}

#[test]
fn shrd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(RBX, Four, 230291700, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(115)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 52, 157, 244, 248, 185, 13, 115], OperandSize::Qword)
}

#[test]
fn shrd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(102)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 219, 102], OperandSize::Word)
}

#[test]
fn shrd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 12662, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(30)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 172, 178, 118, 49, 30], OperandSize::Word)
}

#[test]
fn shrd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(8)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 214, 8], OperandSize::Dword)
}

#[test]
fn shrd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectDisplaced(EAX, 336289007, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 152, 239, 92, 11, 20, 57], OperandSize::Dword)
}

#[test]
fn shrd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(95)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 249, 95], OperandSize::Qword)
}

#[test]
fn shrd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledDisplaced(RDI, Eight, 798570384, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(46)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 172, 36, 253, 144, 55, 153, 47, 46], OperandSize::Qword)
}

#[test]
fn shrd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RDX)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 215, 96], OperandSize::Qword)
}

#[test]
fn shrd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(36)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 172, 9, 36], OperandSize::Qword)
}

#[test]
fn shrd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(SP)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 252], OperandSize::Word)
}

#[test]
fn shrd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 33], OperandSize::Word)
}

#[test]
fn shrd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BX)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 211], OperandSize::Dword)
}

#[test]
fn shrd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(ECX, ESI, Four, 574468261, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 148, 177, 165, 176, 61, 34], OperandSize::Dword)
}

#[test]
fn shrd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(BP)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 229], OperandSize::Qword)
}

#[test]
fn shrd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Indirect(RBX, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 43], OperandSize::Qword)
}

#[test]
fn shrd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 254], OperandSize::Word)
}

#[test]
fn shrd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 173, 16], OperandSize::Word)
}

#[test]
fn shrd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 242], OperandSize::Dword)
}

#[test]
fn shrd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 427466263, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 156, 128, 23, 158, 122, 25], OperandSize::Dword)
}

#[test]
fn shrd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(EDI)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 215], OperandSize::Qword)
}

#[test]
fn shrd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 68551211, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 173, 188, 80, 43, 2, 22, 4], OperandSize::Qword)
}

#[test]
fn shrd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(Direct(RSP)), operand2: Some(Direct(RSI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 244], OperandSize::Qword)
}

#[test]
fn shrd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHRD, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 1275844478, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 173, 156, 142, 126, 215, 11, 76], OperandSize::Qword)
}

