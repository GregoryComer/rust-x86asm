use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphaddw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 113, 1, 220], OperandSize::Dword)
}

#[test]
fn vphaddw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 760134045, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 1, 4, 189, 157, 185, 78, 45], OperandSize::Dword)
}

#[test]
fn vphaddw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 1, 197], OperandSize::Qword)
}

#[test]
fn vphaddw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RCX, Four, 424245202, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 1, 164, 142, 210, 119, 73, 25], OperandSize::Qword)
}

#[test]
fn vphaddw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 1, 226], OperandSize::Dword)
}

#[test]
fn vphaddw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, ESI, Four, 444622206, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 1, 164, 179, 126, 101, 128, 26], OperandSize::Dword)
}

#[test]
fn vphaddw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 1, 224], OperandSize::Qword)
}

#[test]
fn vphaddw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHADDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 600930442, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 1, 148, 143, 138, 120, 209, 35], OperandSize::Qword)
}

