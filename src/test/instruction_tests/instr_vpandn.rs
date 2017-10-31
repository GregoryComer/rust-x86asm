use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpandn_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 223, 233], OperandSize::Dword)
}

#[test]
fn vpandn_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 223, 1], OperandSize::Dword)
}

#[test]
fn vpandn_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 223, 222], OperandSize::Qword)
}

#[test]
fn vpandn_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 223, 39], OperandSize::Qword)
}

#[test]
fn vpandn_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 223, 195], OperandSize::Dword)
}

#[test]
fn vpandn_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 223, 12, 214], OperandSize::Dword)
}

#[test]
fn vpandn_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 223, 197], OperandSize::Qword)
}

#[test]
fn vpandn_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDN, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 223, 36, 129], OperandSize::Qword)
}

