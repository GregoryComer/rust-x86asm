use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmaddsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 182, 222], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM2)), operand3: Some(IndirectDisplaced(ECX, 2074267545, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 182, 185, 153, 207, 162, 123], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 182, 216], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1150739545, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 182, 20, 253, 89, 228, 150, 68], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM4)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 117, 182, 228], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM3)), operand3: Some(Indirect(EBX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 101, 182, 51], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 182, 241], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectDisplaced(RSI, 200187095, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 182, 134, 215, 156, 238, 11], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 93, 139, 182, 222], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 85, 137, 182, 38], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, ESI, Four, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 125, 156, 182, 36, 177], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM20)), operand2: Some(Direct(XMM13)), operand3: Some(Direct(XMM17)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 162, 21, 141, 182, 225], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM27)), operand2: Some(Direct(XMM28)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RBX, Two, 1048623377, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 98, 29, 130, 182, 156, 89, 17, 185, 128, 62], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(XMM23)), operand2: Some(Direct(XMM21)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RSI, Eight, 1385447773, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 226, 85, 148, 182, 188, 246, 93, 65, 148, 82], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 171, 182, 209], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ESI, EDX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 125, 172, 182, 44, 150], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledIndexed(EDX, EBX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 101, 189, 182, 20, 90], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM26)), operand2: Some(Direct(YMM20)), operand3: Some(Direct(YMM24)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 2, 93, 165, 182, 208], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM23)), operand2: Some(Direct(YMM12)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1522395121, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 226, 29, 169, 182, 60, 117, 241, 231, 189, 90], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(YMM13)), operand2: Some(Direct(YMM24)), operand3: Some(IndirectDisplaced(RBX, 1230233148, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 114, 61, 178, 182, 171, 60, 222, 83, 73], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM6)), operand2: Some(Direct(ZMM2)), operand3: Some(Direct(ZMM4)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 109, 153, 182, 244], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM4)), operand3: Some(Indirect(EAX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 93, 201, 182, 8], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM6)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 77, 219, 182, 12, 65], OperandSize::Dword)
}

#[test]
fn vfmaddsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM9)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM27)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 18, 117, 250, 182, 203], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM31)), operand2: Some(Direct(ZMM12)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Eight, 1066478676, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 98, 29, 207, 182, 188, 193, 84, 44, 145, 63], OperandSize::Qword)
}

#[test]
fn vfmaddsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMADDSUB231PS, operand1: Some(Direct(ZMM26)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RDI, Four, 286131924, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 98, 85, 217, 182, 148, 190, 212, 6, 14, 17], OperandSize::Qword)
}

