use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpshuflw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Literal8(12)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 255, 12], OperandSize::Dword)
}

fn vpshuflw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM0)), operand2: Some(Indirect(EBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(126)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 3, 126], OperandSize::Dword)
}

fn vpshuflw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM3)), operand3: Some(Literal8(76)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 203, 76], OperandSize::Qword)
}

fn vpshuflw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectScaledIndexed(RBX, RCX, Four, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 251, 112, 12, 139, 11], OperandSize::Qword)
}

fn vpshuflw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 195, 28], OperandSize::Dword)
}

fn vpshuflw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM6)), operand2: Some(IndirectScaledIndexedDisplaced(EAX, ESI, Four, 1587639549, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 180, 176, 253, 116, 161, 94, 1], OperandSize::Dword)
}

fn vpshuflw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(Literal8(86)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 203, 86], OperandSize::Qword)
}

fn vpshuflw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM3)), operand2: Some(Indirect(RBX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 255, 112, 27, 65], OperandSize::Qword)
}

fn vpshuflw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Literal8(45)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 127, 139, 112, 205, 45], OperandSize::Dword)
}

fn vpshuflw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM1)), operand2: Some(IndirectDisplaced(EAX, 574408485, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(96)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 137, 112, 136, 37, 199, 60, 34, 96], OperandSize::Dword)
}

fn vpshuflw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM27)), operand3: Some(Literal8(65)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 127, 141, 112, 251, 65], OperandSize::Qword)
}

fn vpshuflw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(XMM9)), operand2: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand3: Some(Literal8(31)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 113, 127, 142, 112, 11, 31], OperandSize::Qword)
}

fn vpshuflw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(Literal8(11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 127, 174, 112, 224, 11], OperandSize::Dword)
}

fn vpshuflw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM4)), operand2: Some(IndirectScaledIndexed(EDI, ECX, Eight, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 175, 112, 36, 207, 22], OperandSize::Dword)
}

fn vpshuflw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM13)), operand3: Some(Literal8(119)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 209, 127, 171, 112, 197, 119], OperandSize::Qword)
}

fn vpshuflw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(YMM21)), operand2: Some(Indirect(RAX, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(44)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 127, 170, 112, 40, 44], OperandSize::Qword)
}

fn vpshuflw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Literal8(42)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 127, 201, 112, 241, 42], OperandSize::Dword)
}

fn vpshuflw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM6)), operand2: Some(IndirectScaledIndexed(EDX, EDI, Eight, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(43)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 127, 207, 112, 52, 250, 43], OperandSize::Dword)
}

fn vpshuflw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(99)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 113, 127, 207, 112, 220, 99], OperandSize::Qword)
}

fn vpshuflw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPSHUFLW, operand1: Some(Direct(ZMM22)), operand2: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Eight, 214435646, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(101)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 127, 206, 112, 180, 210, 62, 7, 200, 12, 101], OperandSize::Qword)
}

