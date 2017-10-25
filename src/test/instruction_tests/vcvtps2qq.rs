use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vcvtps2qq_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 142, 123, 223], OperandSize::Dword)
}

fn vcvtps2qq_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM3)), operand2: Some(IndirectScaledDisplaced(ECX, Four, 2019895532, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 143, 123, 28, 141, 236, 40, 101, 120], OperandSize::Dword)
}

fn vcvtps2qq_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 225, 125, 137, 123, 249], OperandSize::Qword)
}

fn vcvtps2qq_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(XMM19)), operand2: Some(IndirectScaledDisplaced(RAX, Eight, 2024452281, Some(OperandSize::Qword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 125, 142, 123, 28, 197, 185, 176, 170, 120], OperandSize::Qword)
}

fn vcvtps2qq_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM2)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 123, 214], OperandSize::Dword)
}

fn vcvtps2qq_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM3)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Four, 557069439, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 125, 173, 123, 156, 144, 127, 52, 52, 33], OperandSize::Dword)
}

fn vcvtps2qq_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM22)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 125, 170, 123, 244], OperandSize::Qword)
}

fn vcvtps2qq_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(YMM18)), operand2: Some(IndirectScaledDisplaced(RDI, Eight, 1201241223, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 225, 125, 173, 123, 20, 253, 135, 124, 153, 71], OperandSize::Qword)
}

fn vcvtps2qq_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 255, 123, 214], OperandSize::Dword)
}

fn vcvtps2qq_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexed(ESI, EDI, Four, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 123, 20, 190], OperandSize::Dword)
}

fn vcvtps2qq_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(YMM29)), operand3: None, operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 1, 125, 188, 123, 229], OperandSize::Qword)
}

fn vcvtps2qq_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VCVTPS2QQ, operand1: Some(Direct(ZMM18)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 125, 207, 123, 19], OperandSize::Qword)
}

