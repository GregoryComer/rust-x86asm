use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttps2dq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 243], OperandSize::Dword)
}

fn vcvttps2dq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EDI, Two, 504606618, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 36, 125, 154, 175, 19, 30], OperandSize::Dword)
}

fn vcvttps2dq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 209], OperandSize::Qword)
}

fn vcvttps2dq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM7)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 501758280, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 250, 91, 60, 205, 72, 57, 232, 29], OperandSize::Qword)
}

fn vcvttps2dq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 215], OperandSize::Dword)
}

fn vcvttps2dq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(EDI, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 7], OperandSize::Dword)
}

fn vcvttps2dq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 230], OperandSize::Qword)
}

fn vcvttps2dq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectDisplaced(RSI, 962380387, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 254, 91, 150, 99, 194, 92, 57], OperandSize::Qword)
}

fn vcvttps2dq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 142, 91, 240], OperandSize::Dword)
}

fn vcvttps2dq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM6)), operand2: Some(IndirectDisplaced(EDI, 301525614, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 126, 137, 91, 183, 110, 234, 248, 17], OperandSize::Dword)
}

fn vcvttps2dq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 145, 126, 142, 91, 229], OperandSize::Qword)
}

fn vcvttps2dq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexed(RBX, RDI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 143, 91, 20, 187], OperandSize::Qword)
}

fn vcvttps2dq_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 175, 91, 235], OperandSize::Dword)
}

fn vcvttps2dq_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM1)), operand2: Some(IndirectDisplaced(ECX, 1152223052, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 174, 91, 137, 76, 135, 173, 68], OperandSize::Dword)
}

fn vcvttps2dq_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 126, 172, 91, 239], OperandSize::Qword)
}

fn vcvttps2dq_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(YMM26)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 126, 172, 91, 19], OperandSize::Qword)
}

fn vcvttps2dq_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 126, 159, 91, 246], OperandSize::Dword)
}

fn vcvttps2dq_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM4)), operand2: Some(IndirectDisplaced(EAX, 1583238507, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 126, 206, 91, 160, 107, 77, 94, 94], OperandSize::Dword)
}

fn vcvttps2dq_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K7), broadcast: None }, &[98, 129, 126, 159, 91, 246], OperandSize::Qword)
}

fn vcvttps2dq_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2DQ, operand1: Some(Direct(ZMM29)), operand2: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 126, 207, 91, 40], OperandSize::Qword)
}

