use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpandnd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 69, 143, 223, 194], OperandSize::Dword)
}

fn vpandnd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 69, 142, 223, 10], OperandSize::Dword)
}

fn vpandnd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 85, 154, 223, 16], OperandSize::Dword)
}

fn vpandnd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM31)), operand3: Some(Direct(XMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 1, 5, 129, 223, 224], OperandSize::Qword)
}

fn vpandnd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM16)), operand3: Some(IndirectDisplaced(RAX, 1068272785, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 125, 132, 223, 128, 145, 140, 172, 63], OperandSize::Qword)
}

fn vpandnd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM22)), operand3: Some(IndirectDisplaced(RSI, 1082604752, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 77, 151, 223, 174, 208, 60, 135, 64], OperandSize::Qword)
}

fn vpandnd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 109, 170, 223, 228], OperandSize::Dword)
}

fn vpandnd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectDisplaced(ECX, 626108400, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 117, 172, 223, 185, 240, 167, 81, 37], OperandSize::Dword)
}

fn vpandnd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 283492379, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 117, 191, 223, 28, 93, 27, 192, 229, 16], OperandSize::Dword)
}

fn vpandnd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM14)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 69, 172, 223, 254], OperandSize::Qword)
}

fn vpandnd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM29)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RDI, RCX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 85, 169, 223, 44, 207], OperandSize::Qword)
}

fn vpandnd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Indirect(RBX, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 125, 190, 223, 59], OperandSize::Qword)
}

fn vpandnd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 125, 206, 223, 215], OperandSize::Dword)
}

fn vpandnd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 859229659, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 101, 207, 223, 20, 221, 219, 205, 54, 51], OperandSize::Dword)
}

fn vpandnd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(EBX, Two, 395463562, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 223, 223, 60, 93, 138, 75, 146, 23], OperandSize::Dword)
}

fn vpandnd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 161, 125, 206, 223, 215], OperandSize::Qword)
}

fn vpandnd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RBX, Eight, 1033075438, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 97, 69, 201, 223, 156, 218, 238, 122, 147, 61], OperandSize::Qword)
}

fn vpandnd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPANDND, operand1: Some(Direct(ZMM28)), operand2: Some(Direct(ZMM26)), operand3: Some(IndirectScaledIndexed(RAX, RDX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 45, 214, 223, 36, 80], OperandSize::Qword)
}

