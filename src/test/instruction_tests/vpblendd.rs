use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpblendd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(31)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 89, 2, 236, 31], OperandSize::Dword)
}

fn vpblendd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(EDX, ECX, Two, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(85)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 121, 2, 4, 74, 85], OperandSize::Dword)
}

fn vpblendd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 2, 240, 72], OperandSize::Qword)
}

fn vpblendd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectDisplaced(RAX, 1668399845, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(69)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 65, 2, 184, 229, 194, 113, 99, 69], OperandSize::Qword)
}

fn vpblendd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(124)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 2, 195, 124], OperandSize::Dword)
}

fn vpblendd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EAX, Eight, 1330919074, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(112)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 93, 2, 148, 193, 162, 54, 84, 79, 112], OperandSize::Dword)
}

fn vpblendd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(11)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 125, 2, 215, 11], OperandSize::Qword)
}

fn vpblendd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPBLENDD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(93)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 117, 2, 3, 93], OperandSize::Qword)
}

