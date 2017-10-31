use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BX)), operand2: Some(Literal8(35)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 251, 35], OperandSize::Word)
}

#[test]
fn btc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(DI, 9, Some(OperandSize::Word), None)), operand2: Some(Literal8(18)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 125, 9, 18], OperandSize::Word)
}

#[test]
fn btc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Literal8(69)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 254, 69], OperandSize::Dword)
}

#[test]
fn btc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EAX, 1407864443, Some(OperandSize::Word), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 184, 123, 78, 234, 83, 125], OperandSize::Dword)
}

#[test]
fn btc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DI)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 255, 54], OperandSize::Qword)
}

#[test]
fn btc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(RDI, Some(OperandSize::Word), None)), operand2: Some(Literal8(92)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 63, 92], OperandSize::Qword)
}

#[test]
fn btc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDI)), operand2: Some(Literal8(90)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 255, 90], OperandSize::Word)
}

#[test]
fn btc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 26427, Some(OperandSize::Dword), None)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 184, 59, 103, 64], OperandSize::Word)
}

#[test]
fn btc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 252, 48], OperandSize::Dword)
}

#[test]
fn btc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(ESI, 424257545, Some(OperandSize::Dword), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 190, 9, 168, 73, 25, 55], OperandSize::Dword)
}

#[test]
fn btc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDX)), operand2: Some(Literal8(54)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 250, 54], OperandSize::Qword)
}

#[test]
fn btc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1238391808, Some(OperandSize::Dword), None)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 60, 149, 0, 92, 208, 73, 17], OperandSize::Qword)
}

#[test]
fn btc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RDX)), operand2: Some(Literal8(59)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 250, 59], OperandSize::Qword)
}

#[test]
fn btc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(RDX, Four, 1833667555, Some(OperandSize::Qword), None)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 60, 149, 227, 139, 75, 109, 42], OperandSize::Qword)
}

#[test]
fn btc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 255], OperandSize::Word)
}

#[test]
fn btc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(BP, 129, Some(OperandSize::Word), None)), operand2: Some(Direct(CX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 142, 129, 0], OperandSize::Word)
}

#[test]
fn btc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(CX)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 241], OperandSize::Dword)
}

#[test]
fn btc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EAX, 387689954, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 144, 226, 173, 27, 23], OperandSize::Dword)
}

#[test]
fn btc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 254], OperandSize::Qword)
}

#[test]
fn btc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RDX, RCX, Eight, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 52, 202], OperandSize::Qword)
}

#[test]
fn btc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 203], OperandSize::Word)
}

#[test]
fn btc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(SI, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 44], OperandSize::Word)
}

#[test]
fn btc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 251], OperandSize::Dword)
}

#[test]
fn btc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Two, 694273384, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 148, 90, 104, 197, 97, 41], OperandSize::Dword)
}

#[test]
fn btc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESP)), operand2: Some(Direct(ECX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 204], OperandSize::Qword)
}

#[test]
fn btc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RSI, RAX, Four, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 28, 134], OperandSize::Qword)
}

#[test]
fn btc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RBP)), operand2: Some(Direct(RDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 253], OperandSize::Qword)
}

#[test]
fn btc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RDX, RBX, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(RSI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 52, 154], OperandSize::Qword)
}

