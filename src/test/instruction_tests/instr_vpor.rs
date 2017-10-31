use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpor_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 233, 235, 223], OperandSize::Dword)
}

#[test]
fn vpor_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 2135019820, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 235, 20, 93, 44, 209, 65, 127], OperandSize::Dword)
}

#[test]
fn vpor_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 235, 254], OperandSize::Qword)
}

#[test]
fn vpor_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 235, 23], OperandSize::Qword)
}

#[test]
fn vpor_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 235, 231], OperandSize::Dword)
}

#[test]
fn vpor_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ECX, EDI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 235, 60, 121], OperandSize::Dword)
}

#[test]
fn vpor_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 235, 221], OperandSize::Qword)
}

#[test]
fn vpor_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPOR, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(RDI, RSI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 235, 12, 183], OperandSize::Qword)
}

