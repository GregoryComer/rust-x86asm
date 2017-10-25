use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpmaskmovd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectDisplaced(ECX, 806028729, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 140, 169, 185, 5, 11, 48], OperandSize::Dword)
}

fn vpmaskmovd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 140, 1], OperandSize::Qword)
}

fn vpmaskmovd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectDisplaced(ECX, 593187922, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 140, 185, 82, 84, 91, 35], OperandSize::Dword)
}

fn vpmaskmovd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(RSI, RSI, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 140, 60, 118], OperandSize::Qword)
}

fn vpmaskmovd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 174584932, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 142, 4, 213, 100, 244, 103, 10], OperandSize::Dword)
}

fn vpmaskmovd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexed(RSI, RBX, Four, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 142, 20, 158], OperandSize::Qword)
}

fn vpmaskmovd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledDisplaced(EDX, Eight, 680552270, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 93, 142, 44, 213, 78, 103, 144, 40], OperandSize::Dword)
}

fn vpmaskmovd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPMASKMOVD, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 142, 20, 210], OperandSize::Qword)
}

