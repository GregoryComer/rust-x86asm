use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SP)), operand2: Some(Literal8(28)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 252, 28], OperandSize::Word)
}

#[test]
fn btc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(BP, 1801, Some(OperandSize::Word), None)), operand2: Some(Literal8(109)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 190, 9, 7, 109], OperandSize::Word)
}

#[test]
fn btc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SP)), operand2: Some(Literal8(66)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 252, 66], OperandSize::Dword)
}

#[test]
fn btc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 619033428, Some(OperandSize::Word), None)), operand2: Some(Literal8(108)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 60, 253, 84, 179, 229, 36, 108], OperandSize::Dword)
}

#[test]
fn btc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BP)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 253, 104], OperandSize::Qword)
}

#[test]
fn btc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Word), None)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 60, 95, 14], OperandSize::Qword)
}

#[test]
fn btc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 251, 31], OperandSize::Word)
}

#[test]
fn btc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(BX, 2909, Some(OperandSize::Dword), None)), operand2: Some(Literal8(77)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 191, 93, 11, 77], OperandSize::Word)
}

#[test]
fn btc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Literal8(50)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 251, 50], OperandSize::Dword)
}

#[test]
fn btc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(EDI, Two, 326804862, Some(OperandSize::Dword), None)), operand2: Some(Literal8(86)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 60, 125, 126, 165, 122, 19, 86], OperandSize::Dword)
}

#[test]
fn btc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(103)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 252, 103], OperandSize::Qword)
}

#[test]
fn btc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(RSI, Some(OperandSize::Dword), None)), operand2: Some(Literal8(43)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 62, 43], OperandSize::Qword)
}

#[test]
fn btc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RCX)), operand2: Some(Literal8(70)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 249, 70], OperandSize::Qword)
}

#[test]
fn btc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(RDX, RDI, Eight, 1184160069, Some(OperandSize::Qword), None)), operand2: Some(Literal8(62)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 188, 250, 69, 217, 148, 70, 62], OperandSize::Qword)
}

#[test]
fn btc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(BP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 238], OperandSize::Word)
}

#[test]
fn btc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 47, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 73, 47], OperandSize::Word)
}

#[test]
fn btc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BX)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 219], OperandSize::Dword)
}

#[test]
fn btc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EDX, 1606641475, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 154, 67, 103, 195, 95], OperandSize::Dword)
}

#[test]
fn btc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DX)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 210], OperandSize::Qword)
}

#[test]
fn btc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RAX, RSI, Four, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 20, 176], OperandSize::Qword)
}

#[test]
fn btc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 227], OperandSize::Word)
}

#[test]
fn btc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 50], OperandSize::Word)
}

#[test]
fn btc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDX)), operand2: Some(Direct(ESI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 242], OperandSize::Dword)
}

#[test]
fn btc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Four, 830898766, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 188, 191, 78, 130, 134, 49], OperandSize::Dword)
}

#[test]
fn btc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 214], OperandSize::Qword)
}

#[test]
fn btc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RCX, 1602649178, Some(OperandSize::Dword), None)), operand2: Some(Direct(ESP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 161, 90, 124, 134, 95], OperandSize::Qword)
}

#[test]
fn btc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RSI)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 246], OperandSize::Qword)
}

#[test]
fn btc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RDX, Eight, 23053662, Some(OperandSize::Qword), None)), operand2: Some(Direct(RBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 44, 213, 94, 197, 95, 1], OperandSize::Qword)
}

