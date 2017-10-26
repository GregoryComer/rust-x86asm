use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn btc_1() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BX)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 251, 51], OperandSize::Word)
}

#[test]
fn btc_2() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(BX, DI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(67)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 57, 67], OperandSize::Word)
}

#[test]
fn btc_3() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BX)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 251, 2], OperandSize::Dword)
}

#[test]
fn btc_4() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EBX, 2141936693, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 187, 53, 92, 171, 127, 1], OperandSize::Dword)
}

#[test]
fn btc_5() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(BP)), operand2: Some(Literal8(53)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 253, 53], OperandSize::Qword)
}

#[test]
fn btc_6() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(RDX, RSI, Eight, Some(OperandSize::Word), None)), operand2: Some(Literal8(76)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 60, 242, 76], OperandSize::Qword)
}

#[test]
fn btc_7() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDX)), operand2: Some(Literal8(48)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 250, 48], OperandSize::Word)
}

#[test]
fn btc_8() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Literal8(7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 186, 56, 7], OperandSize::Word)
}

#[test]
fn btc_9() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESP)), operand2: Some(Literal8(2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 252, 2], OperandSize::Dword)
}

#[test]
fn btc_10() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(EBX, 109729381, Some(OperandSize::Dword), None)), operand2: Some(Literal8(55)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 187, 101, 86, 138, 6, 55], OperandSize::Dword)
}

#[test]
fn btc_11() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBP)), operand2: Some(Literal8(79)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 253, 79], OperandSize::Qword)
}

#[test]
fn btc_12() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RCX, 2007843256, Some(OperandSize::Dword), None)), operand2: Some(Literal8(96)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 186, 185, 184, 65, 173, 119, 96], OperandSize::Qword)
}

#[test]
fn btc_13() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RDI)), operand2: Some(Literal8(65)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 255, 65], OperandSize::Qword)
}

#[test]
fn btc_14() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RDI, Eight, 791118153, Some(OperandSize::Qword), None)), operand2: Some(Literal8(121)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 186, 188, 248, 73, 129, 39, 47, 121], OperandSize::Qword)
}

#[test]
fn btc_15() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(DI)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 215], OperandSize::Word)
}

#[test]
fn btc_16() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 170, Some(OperandSize::Word), None)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 176, 170, 0], OperandSize::Word)
}

#[test]
fn btc_17() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SI)), operand2: Some(Direct(DI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 254], OperandSize::Dword)
}

#[test]
fn btc_18() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectScaledDisplaced(EBX, Eight, 777963856, Some(OperandSize::Word), None)), operand2: Some(Direct(DX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 20, 221, 80, 201, 94, 46], OperandSize::Dword)
}

#[test]
fn btc_19() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(SP)), operand2: Some(Direct(SI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 244], OperandSize::Qword)
}

#[test]
fn btc_20() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(RSI, Some(OperandSize::Word), None)), operand2: Some(Direct(BX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 30], OperandSize::Qword)
}

#[test]
fn btc_21() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EDI)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 239], OperandSize::Word)
}

#[test]
fn btc_22() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(BP, 203, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 15, 187, 158, 203, 0], OperandSize::Word)
}

#[test]
fn btc_23() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(ESI)), operand2: Some(Direct(EDI)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 254], OperandSize::Dword)
}

#[test]
fn btc_24() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand2: Some(Direct(EBP)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 43], OperandSize::Dword)
}

#[test]
fn btc_25() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(EBX)), operand2: Some(Direct(EBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 219], OperandSize::Qword)
}

#[test]
fn btc_26() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RDI, 1802523545, Some(OperandSize::Dword), None)), operand2: Some(Direct(EDX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[15, 187, 151, 153, 83, 112, 107], OperandSize::Qword)
}

#[test]
fn btc_27() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(Direct(RBX)), operand2: Some(Direct(RBX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 219], OperandSize::Qword)
}

#[test]
fn btc_28() {
    run_test(&Instruction { mnemonic: Mnemonic::BTC, operand1: Some(IndirectDisplaced(RCX, 17190650, Some(OperandSize::Qword), None)), operand2: Some(Direct(RCX)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 15, 187, 137, 250, 78, 6, 1], OperandSize::Qword)
}

