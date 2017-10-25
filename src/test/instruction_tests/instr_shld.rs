use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SI)), operand2: Some(Direct(SI)), operand3: Some(Literal8(80)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 246, 80], OperandSize::Word)
}

#[test]
fn shld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: Some(Literal8(73)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 52, 73], OperandSize::Word)
}

#[test]
fn shld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SP)), operand2: Some(Direct(SP)), operand3: Some(Literal8(32)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 228, 32], OperandSize::Dword)
}

#[test]
fn shld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(EDI, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(62)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 23, 62], OperandSize::Dword)
}

#[test]
fn shld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(BX)), operand3: Some(Literal8(15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 218, 15], OperandSize::Qword)
}

#[test]
fn shld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 20, 182, 98], OperandSize::Qword)
}

#[test]
fn shld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(54)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 217, 54], OperandSize::Word)
}

#[test]
fn shld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(BX, 123, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(92)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 127, 123, 92], OperandSize::Word)
}

#[test]
fn shld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(87)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 254, 87], OperandSize::Dword)
}

#[test]
fn shld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(ESI, Two, 222083443, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(89)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 52, 117, 115, 185, 60, 13, 89], OperandSize::Dword)
}

#[test]
fn shld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 225, 41], OperandSize::Qword)
}

#[test]
fn shld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RDX, RBX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 20, 218, 111], OperandSize::Qword)
}

#[test]
fn shld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RDI)), operand2: Some(Direct(RBX)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 223, 110], OperandSize::Qword)
}

#[test]
fn shld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: Some(Literal8(23)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 9, 23], OperandSize::Qword)
}

#[test]
fn shld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 210], OperandSize::Word)
}

#[test]
fn shld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 100, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 97, 100], OperandSize::Word)
}

#[test]
fn shld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(CX)), operand2: Some(Direct(DX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 209], OperandSize::Dword)
}

#[test]
fn shld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(ESI, 1504964947, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 190, 83, 241, 179, 89], OperandSize::Dword)
}

#[test]
fn shld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(CX)), operand2: Some(Direct(BX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 217], OperandSize::Qword)
}

#[test]
fn shld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 36, 134], OperandSize::Qword)
}

#[test]
fn shld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 237], OperandSize::Word)
}

#[test]
fn shld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(BP, 9290, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 150, 74, 36], OperandSize::Word)
}

#[test]
fn shld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ESI)), operand2: Some(Direct(ESP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 230], OperandSize::Dword)
}

#[test]
fn shld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 265232858, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 36, 253, 218, 33, 207, 15], OperandSize::Dword)
}

#[test]
fn shld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ECX)), operand2: Some(Direct(ECX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 201], OperandSize::Qword)
}

#[test]
fn shld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Eight, 1060062996, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 188, 207, 20, 71, 47, 63], OperandSize::Qword)
}

#[test]
fn shld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 222], OperandSize::Qword)
}

#[test]
fn shld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1781569223, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 20, 77, 199, 150, 48, 106], OperandSize::Qword)
}

