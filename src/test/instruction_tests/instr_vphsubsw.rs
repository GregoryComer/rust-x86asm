use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 7, 243], OperandSize::Dword)
}

#[test]
fn vphsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EDI, Four, 1897437248, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 7, 180, 185, 64, 152, 24, 113], OperandSize::Dword)
}

#[test]
fn vphsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 7, 208], OperandSize::Qword)
}

#[test]
fn vphsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectDisplaced(RSI, 882115532, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 7, 158, 204, 3, 148, 52], OperandSize::Qword)
}

#[test]
fn vphsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 7, 229], OperandSize::Dword)
}

#[test]
fn vphsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EAX, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 7, 20, 184], OperandSize::Dword)
}

#[test]
fn vphsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 7, 220], OperandSize::Qword)
}

#[test]
fn vphsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(RDX, Two, 1401054253, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 7, 36, 85, 45, 100, 130, 83], OperandSize::Qword)
}

