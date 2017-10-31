use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bts_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 213], OperandSize::Word)
}

#[test]
fn bts_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 17269, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 146, 117, 67], OperandSize::Word)
}

#[test]
fn bts_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 231], OperandSize::Dword)
}

#[test]
fn bts_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(EDX, EAX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 60, 66], OperandSize::Dword)
}

#[test]
fn bts_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 213], OperandSize::Qword)
}

#[test]
fn bts_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RSI, RDX, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 36, 150], OperandSize::Qword)
}

#[test]
fn bts_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 227], OperandSize::Word)
}

#[test]
fn bts_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 12747, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 137, 203, 49], OperandSize::Word)
}

#[test]
fn bts_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 222], OperandSize::Dword)
}

#[test]
fn bts_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(ESI, Four, 1683397543, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 44, 181, 167, 155, 86, 100], OperandSize::Dword)
}

#[test]
fn bts_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ECX)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 233], OperandSize::Qword)
}

#[test]
fn bts_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RCX, RCX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 36, 201], OperandSize::Qword)
}

#[test]
fn bts_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RCX)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 241], OperandSize::Qword)
}

#[test]
fn bts_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(RCX, Two, 1550082858, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 36, 77, 42, 99, 100, 92], OperandSize::Qword)
}

#[test]
fn bts_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 235, 14], OperandSize::Word)
}

#[test]
fn bts_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 11336, Some(OperandSize::Word), None)), operand2: Some(Literal8(3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 172, 72, 44, 3], OperandSize::Word)
}

#[test]
fn bts_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Literal8(16)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 235, 16], OperandSize::Dword)
}

#[test]
fn bts_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(ESI, Eight, 1476640558, Some(OperandSize::Word), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 44, 245, 46, 191, 3, 88, 18], OperandSize::Dword)
}

#[test]
fn bts_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 79], OperandSize::Qword)
}

#[test]
fn bts_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(98)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 47, 98], OperandSize::Qword)
}

#[test]
fn bts_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDX)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 234, 34], OperandSize::Word)
}

#[test]
fn bts_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(DI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 45, 52], OperandSize::Word)
}

#[test]
fn bts_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESP)), operand2: Some(Literal8(23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 236, 23], OperandSize::Dword)
}

#[test]
fn bts_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EDI, Four, 366385891, Some(OperandSize::Dword), None)), operand2: Some(Literal8(114)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 172, 186, 227, 154, 214, 21, 114], OperandSize::Dword)
}

#[test]
fn bts_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBX)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 235, 108], OperandSize::Qword)
}

#[test]
fn bts_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 44, 134, 115], OperandSize::Qword)
}

#[test]
fn bts_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RCX)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 233, 77], OperandSize::Qword)
}

#[test]
fn bts_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 44, 202, 18], OperandSize::Qword)
}

