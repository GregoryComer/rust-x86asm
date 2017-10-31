use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 182, 195], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Eight, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 182, 20, 217], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 182, 226], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(IndirectScaledDisplaced(RAX, Two, 677491039, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 182, 12, 69, 95, 177, 97, 40], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 182, 196], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EBX, EDI, Eight, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 229, 182, 12, 251], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 182, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(RCX, Four, 2082777409, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 182, 28, 141, 65, 169, 36, 124], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 245, 139, 182, 193], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 141, 182, 9], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM4)), operand3: Some(IndirectDisplaced(EBX, 176423285, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 157, 182, 139, 117, 1, 132, 10], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 205, 138, 182, 214], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM10)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 518793348, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 173, 138, 182, 28, 77, 132, 40, 236, 30], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectDisplaced(RBX, 759726240, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 229, 151, 182, 163, 160, 128, 72, 45], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 169, 182, 197], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, EDX, Two, 1878004241, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 237, 175, 182, 148, 82, 17, 18, 240, 111], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 2039730379, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 253, 188, 182, 52, 253, 203, 208, 147, 121], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM19)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 229, 167, 182, 200], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM17)), operand2: Some(Direct(YMM20)), operand3: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 221, 166, 182, 15], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RBX, RAX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 157, 179, 182, 20, 67], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 237, 158, 182, 238], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Eight, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 229, 205, 182, 28, 193], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 221, 182, 28, 131], OperandSize::Dword)
}

#[test]
fn vfmaddsub231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 237, 222, 182, 200], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 1591787465, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 229, 204, 182, 52, 181, 201, 191, 224, 94], OperandSize::Qword)
}

#[test]
fn vfmaddsub231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PD, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 545459913, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 245, 221, 182, 60, 157, 201, 14, 131, 32], OperandSize::Qword)
}

