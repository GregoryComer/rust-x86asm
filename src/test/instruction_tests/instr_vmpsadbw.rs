use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vmpsadbw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM3)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 66, 195, 13], OperandSize::Dword)
}

#[test]
fn vmpsadbw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledIndexed(EAX, ECX, Eight, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(56)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 113, 66, 60, 200, 56], OperandSize::Dword)
}

#[test]
fn vmpsadbw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 66, 253, 50], OperandSize::Qword)
}

#[test]
fn vmpsadbw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(RAX, 1162436320, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 105, 66, 176, 224, 94, 73, 69, 125], OperandSize::Qword)
}

#[test]
fn vmpsadbw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 66, 232, 112], OperandSize::Dword)
}

#[test]
fn vmpsadbw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 553027156, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 66, 44, 245, 84, 134, 246, 32, 51], OperandSize::Dword)
}

#[test]
fn vmpsadbw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(58)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 66, 237, 58], OperandSize::Qword)
}

#[test]
fn vmpsadbw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMPSADBW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexed(RBX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(125)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 66, 4, 211, 125], OperandSize::Qword)
}

