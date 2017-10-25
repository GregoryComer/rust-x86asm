use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtuqq2ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 255, 137, 122, 207], OperandSize::Dword)
}

fn vcvtuqq2ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 271738961, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 255, 138, 122, 36, 133, 81, 104, 50, 16], OperandSize::Dword)
}

fn vcvtuqq2ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 129, 255, 139, 122, 221], OperandSize::Qword)
}

fn vcvtuqq2ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM28)), operand2: Some(IndirectScaledDisplaced(RCX, Eight, 839673937, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 255, 143, 122, 36, 205, 81, 104, 12, 50], OperandSize::Qword)
}

fn vcvtuqq2ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(YMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 255, 171, 122, 235], OperandSize::Dword)
}

fn vcvtuqq2ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Two, 1865275633, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 174, 122, 156, 66, 241, 216, 45, 111], OperandSize::Dword)
}

fn vcvtuqq2ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(YMM9)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 209, 255, 170, 122, 209], OperandSize::Qword)
}

fn vcvtuqq2ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(XMM15)), operand2: Some(IndirectDisplaced(RAX, 767872863, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 113, 255, 170, 122, 184, 95, 207, 196, 45], OperandSize::Qword)
}

fn vcvtuqq2ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(ZMM3)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 255, 253, 122, 227], OperandSize::Dword)
}

fn vcvtuqq2ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDX, Two, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 255, 206, 122, 20, 86], OperandSize::Dword)
}

fn vcvtuqq2ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(ZMM0)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 97, 255, 251, 122, 208], OperandSize::Qword)
}

fn vcvtuqq2ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTUQQ2PS, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 255, 207, 122, 28, 182], OperandSize::Qword)
}

