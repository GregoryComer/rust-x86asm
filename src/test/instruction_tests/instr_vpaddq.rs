use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpaddq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 212, 214], OperandSize::Dword)
}

#[test]
fn vpaddq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 1232738451, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 212, 158, 147, 24, 122, 73], OperandSize::Dword)
}

#[test]
fn vpaddq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 249, 212, 224], OperandSize::Qword)
}

#[test]
fn vpaddq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 217, 212, 25], OperandSize::Qword)
}

#[test]
fn vpaddq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 212, 201], OperandSize::Dword)
}

#[test]
fn vpaddq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectDisplaced(ECX, 2123472872, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 237, 212, 129, 232, 159, 145, 126], OperandSize::Dword)
}

#[test]
fn vpaddq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 212, 215], OperandSize::Qword)
}

#[test]
fn vpaddq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RBX, Two, 985183887, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 221, 212, 20, 93, 143, 182, 184, 58], OperandSize::Qword)
}

#[test]
fn vpaddq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 237, 142, 212, 197], OperandSize::Dword)
}

#[test]
fn vpaddq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(ECX, Two, 77030269, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 197, 137, 212, 28, 77, 125, 99, 151, 4], OperandSize::Dword)
}

#[test]
fn vpaddq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(EDI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 253, 155, 212, 63], OperandSize::Dword)
}

#[test]
fn vpaddq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM19)), operand3: Some(Direct(XMM28)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 1, 229, 130, 212, 196], OperandSize::Qword)
}

#[test]
fn vpaddq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 245, 139, 212, 51], OperandSize::Qword)
}

#[test]
fn vpaddq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPADDQ, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM20)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 451352221, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 241, 221, 148, 212, 12, 117, 157, 22, 231, 26], OperandSize::Qword)
}

