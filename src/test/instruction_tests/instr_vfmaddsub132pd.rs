use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub132pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 150, 234], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 150, 1], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 150, 237], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(RBX, Eight, 1210115661, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 150, 44, 221, 77, 230, 32, 72], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 150, 255], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Four, 1358600125, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 197, 150, 4, 133, 189, 151, 250, 80], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 237, 150, 232], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1858963473, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 150, 20, 157, 17, 136, 205, 110], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 221, 139, 150, 198], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Four, 1277580028, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 253, 142, 150, 28, 189, 252, 82, 38, 76], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ESI, 1116307922, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 237, 153, 150, 134, 210, 129, 137, 66], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM12)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 157, 140, 150, 218], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM23)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RBX, Eight, 96435359, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 197, 132, 150, 164, 216, 159, 124, 191, 5], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(XMM9)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 980269376, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 114, 181, 153, 150, 140, 70, 64, 185, 109, 58], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 253, 170, 150, 218], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 237, 171, 150, 60, 129], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Four, 1495122376, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 186, 150, 44, 149, 200, 193, 29, 89], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM18)), operand2: Some(Direct(YMM9)), operand3: Some(Direct(YMM13)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 194, 181, 175, 150, 213], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM23)), operand3: Some(IndirectDisplaced(RDX, 1328226432, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 161, 150, 138, 128, 32, 43, 79], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM14)), operand3: Some(Indirect(RAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 141, 191, 150, 16], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 156, 150, 214], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 346731830, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 253, 204, 150, 4, 125, 54, 181, 170, 20], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 197, 218, 150, 57], OperandSize::Dword)
}

#[test]
fn vfmaddsub132pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM13)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Up), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 210, 149, 220, 150, 243], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM22)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 966505411, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 98, 205, 193, 150, 148, 198, 195, 179, 155, 57], OperandSize::Qword)
}

#[test]
fn vfmaddsub132pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB132PD, operand1: Some(Direct(ZMM11)), operand2: Some(Direct(ZMM7)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 197, 217, 150, 25], OperandSize::Qword)
}

