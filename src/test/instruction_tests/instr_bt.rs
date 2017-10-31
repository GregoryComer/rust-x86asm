use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn bt_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(SI)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 206], OperandSize::Word)
}

#[test]
fn bt_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(BX, Some(OperandSize::Word), None)), operand2: Some(Direct(SP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 39], OperandSize::Word)
}

#[test]
fn bt_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 245], OperandSize::Dword)
}

#[test]
fn bt_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 859152244, Some(OperandSize::Word), None)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 44, 213, 116, 159, 53, 51], OperandSize::Dword)
}

#[test]
fn bt_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 210], OperandSize::Qword)
}

#[test]
fn bt_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(RDI, 214813827, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 159, 131, 204, 205, 12], OperandSize::Qword)
}

#[test]
fn bt_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 205], OperandSize::Word)
}

#[test]
fn bt_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(BP, DI, One, 13550, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 163, 171, 238, 52], OperandSize::Word)
}

#[test]
fn bt_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EBP)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 221], OperandSize::Dword)
}

#[test]
fn bt_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(EAX, 633521868, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 144, 204, 198, 194, 37], OperandSize::Dword)
}

#[test]
fn bt_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 254], OperandSize::Qword)
}

#[test]
fn bt_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RCX, Eight, 475957382, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 163, 164, 203, 134, 136, 94, 28], OperandSize::Qword)
}

#[test]
fn bt_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RSI)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 222], OperandSize::Qword)
}

#[test]
fn bt_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(RCX, 1572133038, Some(OperandSize::Qword), None)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 163, 185, 174, 216, 180, 93], OperandSize::Qword)
}

#[test]
fn bt_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BX)), operand2: Some(Literal8(82)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 227, 82], OperandSize::Word)
}

#[test]
fn bt_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 34, 104], OperandSize::Word)
}

#[test]
fn bt_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(DX)), operand2: Some(Literal8(99)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 226, 99], OperandSize::Dword)
}

#[test]
fn bt_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(EAX, 393634454, Some(OperandSize::Word), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 160, 150, 98, 118, 23, 108], OperandSize::Dword)
}

#[test]
fn bt_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(BX)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 227, 69], OperandSize::Qword)
}

#[test]
fn bt_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RDX, Some(OperandSize::Word), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 34, 45], OperandSize::Qword)
}

#[test]
fn bt_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ESP)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 228, 65], OperandSize::Word)
}

#[test]
fn bt_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 35, 76], OperandSize::Word)
}

#[test]
fn bt_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(ECX)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 225, 96], OperandSize::Dword)
}

#[test]
fn bt_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectDisplaced(ESI, 882897616, Some(OperandSize::Dword), None)), operand2: Some(Literal8(22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 166, 208, 242, 159, 52, 22], OperandSize::Dword)
}

#[test]
fn bt_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(EDI)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 231, 108], OperandSize::Qword)
}

#[test]
fn bt_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(84)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 38, 84], OperandSize::Qword)
}

#[test]
fn bt_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(Direct(RBX)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 227, 62], OperandSize::Qword)
}

#[test]
fn bt_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BT, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Four, 1426256163, Some(OperandSize::Qword), None)), operand2: Some(Literal8(63)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 164, 151, 35, 241, 2, 85, 63], OperandSize::Qword)
}

