use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpackuswb_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 250], OperandSize::Dword)
}

fn vpackuswb_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 201, 103, 10], OperandSize::Dword)
}

fn vpackuswb_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 223], OperandSize::Qword)
}

fn vpackuswb_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1247882192, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 103, 12, 69, 208, 43, 97, 74], OperandSize::Qword)
}

fn vpackuswb_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 205, 103, 243], OperandSize::Dword)
}

fn vpackuswb_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 103, 38], OperandSize::Dword)
}

fn vpackuswb_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM3)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 103, 193], OperandSize::Qword)
}

fn vpackuswb_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RBX, 1415358388, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 253, 103, 131, 180, 167, 92, 84], OperandSize::Qword)
}

fn vpackuswb_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 85, 141, 103, 236], OperandSize::Dword)
}

fn vpackuswb_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 1880360849, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 77, 137, 103, 36, 221, 145, 7, 20, 112], OperandSize::Dword)
}

fn vpackuswb_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM11)), operand2: Some(Direct(XMM20)), operand3: Some(Direct(XMM27)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 17, 93, 129, 103, 219], OperandSize::Qword)
}

fn vpackuswb_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(XMM19)), operand2: Some(Direct(XMM23)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 225, 69, 130, 103, 30], OperandSize::Qword)
}

fn vpackuswb_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 103, 198], OperandSize::Dword)
}

fn vpackuswb_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 532054425, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 175, 103, 163, 153, 129, 182, 31], OperandSize::Dword)
}

fn vpackuswb_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM15)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 161, 5, 175, 103, 198], OperandSize::Qword)
}

fn vpackuswb_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM10)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 1396924416, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 45, 172, 103, 20, 69, 0, 96, 67, 83], OperandSize::Qword)
}

fn vpackuswb_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 101, 203, 103, 243], OperandSize::Dword)
}

fn vpackuswb_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Two, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 69, 204, 103, 4, 87], OperandSize::Dword)
}

fn vpackuswb_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM8)), operand3: Some(Direct(ZMM20)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 177, 61, 204, 103, 220], OperandSize::Qword)
}

fn vpackuswb_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSWB, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM8)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1869404029, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 225, 61, 207, 103, 28, 253, 125, 215, 108, 111], OperandSize::Qword)
}

