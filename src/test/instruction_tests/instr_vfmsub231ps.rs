use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfmsub231ps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 105, 186, 221], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, EDI, Two, 2119792333, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 65, 186, 156, 127, 205, 118, 89, 126], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 186, 197], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RCX, Eight, 1810146751, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 97, 186, 20, 205, 191, 165, 228, 107], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 77, 186, 224], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexed(ECX, EBX, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 125, 186, 36, 153], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM3)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 69, 186, 203], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM5)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 85, 186, 2], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 77, 140, 186, 232], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexed(EBX, EDX, Two, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 85, 142, 186, 36, 83], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: Some(Indirect(ESI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 85, 157, 186, 46], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM31)), operand2: Some(Direct(XMM24)), operand3: Some(Direct(XMM30)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 2, 61, 131, 186, 254], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM9)), operand3: Some(IndirectDisplaced(RCX, 162998580, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 226, 53, 142, 186, 137, 52, 41, 183, 9], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM26)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RDX, Two, 1139249039, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 45, 150, 186, 156, 82, 143, 143, 231, 67], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM1)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 117, 175, 186, 193], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Indirect(EAX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 77, 174, 186, 8], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 975666032, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 109, 186, 186, 44, 69, 112, 123, 39, 58], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 117, 175, 186, 239], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM14)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Two, 869567204, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 114, 125, 171, 186, 180, 70, 228, 138, 212, 51], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 617778515, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 77, 190, 186, 60, 181, 83, 141, 210, 36], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Zero), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 85, 252, 186, 207], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectDisplaced(EBX, 1010891800, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 85, 203, 186, 155, 24, 252, 64, 60], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectDisplaced(ECX, 1295923370, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 242, 109, 218, 186, 153, 170, 56, 62, 77], OperandSize::Dword)
}

#[test]
fn vfmsub231ps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM19)), operand2: Some(Direct(ZMM20)), operand3: Some(Direct(ZMM7)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Down), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 226, 93, 181, 186, 223], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM10)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RSI, Two, 1342479415, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 114, 45, 206, 186, 188, 114, 55, 156, 4, 80], OperandSize::Qword)
}

#[test]
fn vfmsub231ps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFMSUB231PS, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectDisplaced(RAX, 1448559050, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 114, 61, 210, 186, 184, 202, 65, 87, 86], OperandSize::Qword)
}

