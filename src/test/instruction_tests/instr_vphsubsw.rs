use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vphsubsw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 81, 7, 231], OperandSize::Dword)
}

#[test]
fn vphsubsw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ECX, Two, 1360001611, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 7, 156, 79, 75, 250, 15, 81], OperandSize::Dword)
}

#[test]
fn vphsubsw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 7, 222], OperandSize::Qword)
}

#[test]
fn vphsubsw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexed(RCX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 7, 28, 241], OperandSize::Qword)
}

#[test]
fn vphsubsw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 7, 198], OperandSize::Dword)
}

#[test]
fn vphsubsw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(EDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 7, 50], OperandSize::Dword)
}

#[test]
fn vphsubsw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 109, 7, 243], OperandSize::Qword)
}

#[test]
fn vphsubsw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPHSUBSW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(RDX, 995538468, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 7, 130, 36, 182, 86, 59], OperandSize::Qword)
}

