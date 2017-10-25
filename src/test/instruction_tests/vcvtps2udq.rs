use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2udq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 124, 138, 121, 249], OperandSize::Dword)
}

fn vcvtps2udq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM4)), operand2: Some(IndirectScaledIndexed(ESI, EAX, Two, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 141, 121, 36, 70], OperandSize::Dword)
}

fn vcvtps2udq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM22)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 177, 124, 139, 121, 198], OperandSize::Qword)
}

fn vcvtps2udq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(XMM31)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Eight, 2059588666, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 124, 137, 121, 188, 217, 58, 212, 194, 122], OperandSize::Qword)
}

fn vcvtps2udq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 121, 242], OperandSize::Dword)
}

fn vcvtps2udq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(ESI, Two, 649681383, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 121, 4, 117, 231, 89, 185, 38], OperandSize::Dword)
}

fn vcvtps2udq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM19)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 33, 124, 174, 121, 235], OperandSize::Qword)
}

fn vcvtps2udq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(YMM8)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 124, 172, 121, 0], OperandSize::Qword)
}

fn vcvtps2udq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 255, 121, 209], OperandSize::Dword)
}

fn vcvtps2udq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM2)), operand2: Some(Indirect(EDI, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 121, 23], OperandSize::Dword)
}

fn vcvtps2udq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 161, 124, 252, 121, 223], OperandSize::Qword)
}

fn vcvtps2udq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2UDQ, operand1: Some(Direct(ZMM12)), operand2: Some(IndirectScaledIndexed(RSI, RDI, Eight, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 113, 124, 201, 121, 36, 254], OperandSize::Qword)
}

