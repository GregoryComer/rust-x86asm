use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 150, 245], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexed(EDX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 150, 36, 130], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 233, 150, 213], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1068474159, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 150, 188, 65, 47, 159, 175, 63], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 150, 255], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(EDI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 150, 44, 151], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 150, 219], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 300749378, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 213, 150, 20, 221, 66, 18, 237, 17], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 142, 150, 243], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 213, 137, 150, 46], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Four, 1283944726, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 253, 159, 150, 164, 150, 22, 113, 135, 76], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM10)), operand3: Some(Direct(XMM29)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 2, 173, 142, 150, 197], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM24)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Four, 175051868, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 98, 197, 131, 150, 132, 182, 92, 20, 111, 10], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM18)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RDX, Two, 1577529961, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 226, 253, 156, 150, 148, 80, 105, 50, 7, 94], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 205, 174, 150, 244], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Two, 416130846, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 172, 150, 180, 86, 30, 167, 205, 24], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EDX, Eight, 2142426645, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 185, 150, 164, 208, 21, 214, 178, 127], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM16)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 210, 253, 165, 150, 248], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1356560790, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 114, 173, 165, 150, 148, 89, 150, 121, 219, 80], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM9)), operand3: Some(IndirectScaledIndexed(RDX, RDX, Two, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 181, 190, 150, 36, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM5)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 213, 157, 150, 245], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Indirect(ECX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 204, 150, 1], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EBX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 221, 222, 150, 19], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM23)), operand2: Some(Direct(ZMM23)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 197, 246, 150, 252], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM17)), operand3: Some(Indirect(RDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 242, 245, 197, 150, 58], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM16)), operand2: Some(Direct(ZMM31)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 843699453, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 133, 210, 150, 4, 125, 253, 212, 73, 50], OperandSize::Qword)
}

