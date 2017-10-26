use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bts_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 227], OperandSize::Word)
}

#[test]
fn bts_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 26], OperandSize::Word)
}

#[test]
fn bts_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(SI)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 230], OperandSize::Dword)
}

#[test]
fn bts_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(EBX, 675708953, Some(OperandSize::Word), None)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 187, 25, 128, 70, 40], OperandSize::Dword)
}

#[test]
fn bts_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BX)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 251], OperandSize::Qword)
}

#[test]
fn bts_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(RSI, 959052989, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 158, 189, 252, 41, 57], OperandSize::Qword)
}

#[test]
fn bts_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EDX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 250], OperandSize::Word)
}

#[test]
fn bts_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Memory(25765, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 171, 22, 165, 100], OperandSize::Word)
}

#[test]
fn bts_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 219], OperandSize::Dword)
}

#[test]
fn bts_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexed(EDX, EDX, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 12, 210], OperandSize::Dword)
}

#[test]
fn bts_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 220], OperandSize::Qword)
}

#[test]
fn bts_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 171, 31], OperandSize::Qword)
}

#[test]
fn bts_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 235], OperandSize::Qword)
}

#[test]
fn bts_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 2140615983, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 171, 188, 198, 47, 53, 151, 127], OperandSize::Qword)
}

#[test]
fn bts_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(SP)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 236, 115], OperandSize::Word)
}

#[test]
fn bts_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectDisplaced(SI, 209, Some(OperandSize::Word), None)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 172, 209, 0, 93], OperandSize::Word)
}

#[test]
fn bts_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(BP)), operand2: Some(Literal8(49)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 49], OperandSize::Dword)
}

#[test]
fn bts_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(EDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(127)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 42, 127], OperandSize::Dword)
}

#[test]
fn bts_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(DX)), operand2: Some(Literal8(4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 234, 4], OperandSize::Qword)
}

#[test]
fn bts_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RAX, Eight, 1609054989, Some(OperandSize::Word), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 172, 199, 13, 59, 232, 95, 63], OperandSize::Qword)
}

#[test]
fn bts_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(EBP)), operand2: Some(Literal8(105)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 237, 105], OperandSize::Word)
}

#[test]
fn bts_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 40, Some(OperandSize::Dword), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 105, 40, 125], OperandSize::Word)
}

#[test]
fn bts_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 238, 29], OperandSize::Dword)
}

#[test]
fn bts_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 46, 24], OperandSize::Dword)
}

#[test]
fn bts_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(ESI)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 238, 62], OperandSize::Qword)
}

#[test]
fn bts_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(IndirectScaledDisplaced(RCX, Four, 1162199374, Some(OperandSize::Dword), None)), operand2: Some(Literal8(37)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 44, 141, 78, 193, 69, 69, 37], OperandSize::Qword)
}

#[test]
fn bts_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Direct(RSP)), operand2: Some(Literal8(39)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 236, 39], OperandSize::Qword)
}

#[test]
fn bts_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTS, operand1: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(115)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 40, 115], OperandSize::Qword)
}

