use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn shld_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: Some(Literal8(110)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 206, 110], OperandSize::Word)
}

#[test]
fn shld_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 5869, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 152, 237, 22, 57], OperandSize::Word)
}

#[test]
fn shld_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(DX)), operand2: Some(Direct(SP)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 226, 99], OperandSize::Dword)
}

#[test]
fn shld_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(EDI, Two, 13679422, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 44, 125, 62, 187, 208, 0, 11], OperandSize::Dword)
}

#[test]
fn shld_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(CX)), operand2: Some(Direct(SP)), operand3: Some(Literal8(56)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 225, 56], OperandSize::Qword)
}

#[test]
fn shld_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RDI, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: Some(Literal8(69)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 28, 151, 69], OperandSize::Qword)
}

#[test]
fn shld_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDI)), operand2: Some(Direct(ESI)), operand3: Some(Literal8(25)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 247, 25], OperandSize::Word)
}

#[test]
fn shld_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(57)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 164, 33, 57], OperandSize::Word)
}

#[test]
fn shld_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EDI)), operand3: Some(Literal8(29)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 253, 29], OperandSize::Dword)
}

#[test]
fn shld_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(EBX, 1001494336, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 147, 64, 151, 177, 59, 0], OperandSize::Dword)
}

#[test]
fn shld_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Literal8(106)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 218, 106], OperandSize::Qword)
}

#[test]
fn shld_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: Some(Literal8(81)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 164, 36, 154, 81], OperandSize::Qword)
}

#[test]
fn shld_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RDX)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 234, 7], OperandSize::Qword)
}

#[test]
fn shld_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 164, 41, 59], OperandSize::Qword)
}

#[test]
fn shld_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 206], OperandSize::Word)
}

#[test]
fn shld_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 242, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 184, 242, 0], OperandSize::Word)
}

#[test]
fn shld_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 235], OperandSize::Dword)
}

#[test]
fn shld_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledIndexed(ECX, ESI, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 60, 241], OperandSize::Dword)
}

#[test]
fn shld_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(BX)), operand2: Some(Direct(BP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 235], OperandSize::Qword)
}

#[test]
fn shld_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(RDI, 767264877, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 167, 109, 136, 187, 45], OperandSize::Qword)
}

#[test]
fn shld_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 221], OperandSize::Word)
}

#[test]
fn shld_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectDisplaced(BX, 12427, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 165, 183, 139, 48], OperandSize::Word)
}

#[test]
fn shld_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(EDX)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 218], OperandSize::Dword)
}

#[test]
fn shld_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 26], OperandSize::Dword)
}

#[test]
fn shld_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(ECX)), operand2: Some(Direct(ESI)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 241], OperandSize::Qword)
}

#[test]
fn shld_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1660167201, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 165, 44, 213, 33, 36, 244, 98], OperandSize::Qword)
}

#[test]
fn shld_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSP)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 230], OperandSize::Qword)
}

#[test]
fn shld_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SHLD, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 1665454131, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDX)), operand3: Some(Direct(CL)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 165, 20, 213, 51, 208, 68, 99], OperandSize::Qword)
}

