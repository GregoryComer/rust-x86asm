use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpackusdw_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM0)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 121, 43, 210], OperandSize::Dword)
}

fn vpackusdw_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1106346637, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 89, 43, 20, 197, 141, 130, 241, 65], OperandSize::Dword)
}

fn vpackusdw_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 43, 238], OperandSize::Qword)
}

fn vpackusdw_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(RSI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 73, 43, 62], OperandSize::Qword)
}

fn vpackusdw_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 43, 233], OperandSize::Dword)
}

fn vpackusdw_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectDisplaced(EAX, 1966475815, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 144, 39, 10, 54, 117], OperandSize::Dword)
}

fn vpackusdw_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 43, 225], OperandSize::Qword)
}

fn vpackusdw_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(RCX, RAX, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 43, 28, 193], OperandSize::Qword)
}

fn vpackusdw_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 43, 224], OperandSize::Dword)
}

fn vpackusdw_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(EDX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 69, 137, 43, 26], OperandSize::Dword)
}

fn vpackusdw_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EBX, Eight, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 158, 43, 44, 219], OperandSize::Dword)
}

fn vpackusdw_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM18)), operand3: Some(Direct(XMM21)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 34, 109, 129, 43, 237], OperandSize::Qword)
}

fn vpackusdw_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM12)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RDI, RDX, Two, 863715492, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 114, 101, 135, 43, 164, 87, 164, 64, 123, 51], OperandSize::Qword)
}

fn vpackusdw_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM12)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 152758470, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 29, 154, 43, 60, 197, 198, 232, 26, 9], OperandSize::Qword)
}

fn vpackusdw_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 109, 204, 43, 251], OperandSize::Dword)
}

fn vpackusdw_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 207, 43, 20, 195], OperandSize::Dword)
}

fn vpackusdw_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1472685267, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 220, 43, 180, 67, 211, 100, 199, 87], OperandSize::Dword)
}

fn vpackusdw_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM26)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 226, 45, 195, 43, 206], OperandSize::Qword)
}

fn vpackusdw_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM13)), operand2: Some(Direct(ZMM24)), operand3: Some(Indirect(RBX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 61, 197, 43, 43], OperandSize::Qword)
}

fn vpackusdw_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPACKUSDW, operand1: Some(Direct(ZMM21)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectDisplaced(RAX, 1105004122, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 226, 45, 221, 43, 168, 90, 6, 221, 65], OperandSize::Qword)
}

