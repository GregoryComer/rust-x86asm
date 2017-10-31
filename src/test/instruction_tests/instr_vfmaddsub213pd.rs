use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub213pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 166, 246], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 317391813, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 217, 166, 52, 245, 197, 3, 235, 18], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 166, 232], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(RSI, RDX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 166, 28, 214], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 166, 205], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EAX, Eight, 1807378613, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 166, 28, 197, 181, 104, 186, 107], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 166, 229], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1011668286, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 166, 12, 253, 62, 213, 76, 60], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 141, 166, 203], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 997888088, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 142, 166, 52, 85, 88, 144, 122, 59], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 205, 153, 166, 26], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM9)), operand3: Some(Direct(XMM25)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 2, 181, 143, 166, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 141, 135, 166, 36, 146], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM14)), operand3: Some(IndirectScaledIndexed(RAX, RDI, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 141, 158, 166, 12, 184], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM4)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 171, 166, 202], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(EAX, 1753710470, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 197, 170, 166, 168, 134, 127, 135, 104], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectDisplaced(EBX, 1984193321, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 187, 166, 187, 41, 99, 68, 118], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM12)), operand2: Some(Direct(YMM13)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 18, 149, 175, 166, 224], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM30)), operand2: Some(Direct(YMM19)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 1478918660, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 98, 229, 165, 166, 180, 131, 4, 130, 38, 88], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM18)), operand3: Some(IndirectScaledIndexed(RSI, RCX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 183, 166, 36, 78], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 245, 255, 166, 245], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledIndexed(ESI, ECX, Four, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 207, 166, 28, 142], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 223, 166, 52, 127], OperandSize::Dword)
}

#[test]
fn vfmaddsub213pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM27)), operand2: Some(Direct(ZMM18)), operand3: Some(Direct(ZMM13)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 66, 237, 151, 166, 221], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM30)), operand3: Some(IndirectDisplaced(RDX, 1514135942, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 141, 198, 166, 178, 134, 225, 63, 90], OperandSize::Qword)
}

#[test]
fn vfmaddsub213pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB213PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM17)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1686776686, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 245, 213, 166, 44, 157, 110, 43, 138, 100], OperandSize::Qword)
}

