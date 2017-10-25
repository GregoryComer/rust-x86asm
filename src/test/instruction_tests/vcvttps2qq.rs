use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvttps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 139, 122, 224], OperandSize::Dword)
}

fn vcvttps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledDisplaced(ECX, Two, 92673281, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 141, 122, 36, 77, 1, 21, 134, 5], OperandSize::Dword)
}

fn vcvttps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM8)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 125, 143, 122, 194], OperandSize::Qword)
}

fn vcvttps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(XMM29)), operand2: Some(IndirectScaledIndexed(RDI, RBX, Two, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 141, 122, 44, 95], OperandSize::Qword)
}

fn vcvttps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM4)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 122, 231], OperandSize::Dword)
}

fn vcvttps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM2)), operand2: Some(IndirectScaledIndexed(ESI, ESI, Four, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 174, 122, 20, 182], OperandSize::Dword)
}

fn vcvttps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM18)), operand2: Some(Direct(XMM21)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 161, 125, 171, 122, 213], OperandSize::Qword)
}

fn vcvttps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(YMM30)), operand2: Some(IndirectScaledIndexed(RAX, RSI, Eight, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 97, 125, 173, 122, 52, 240], OperandSize::Qword)
}

fn vcvttps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 157, 122, 202], OperandSize::Dword)
}

fn vcvttps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM7)), operand2: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 125, 202, 122, 57], OperandSize::Dword)
}

fn vcvttps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: true, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 125, 156, 122, 213], OperandSize::Qword)
}

fn vcvttps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTTPS2QQ, operand1: Some(Direct(ZMM3)), operand2: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 2114116653, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 122, 156, 143, 45, 220, 2, 126], OperandSize::Qword)
}

