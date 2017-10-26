use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vpcmpgtd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM5)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 102, 229], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ECX, EBX, Eight, 1191111779, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 209, 102, 156, 217, 99, 236, 254, 70], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 241, 102, 247], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RSI, RBX, Eight, 1096399820, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 193, 102, 156, 222, 204, 187, 89, 65], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 197, 102, 237], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 248768154, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 102, 20, 125, 154, 230, 211, 14], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 213, 102, 254], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM3)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 1748786281, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 229, 102, 36, 117, 105, 92, 60, 104], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM5)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 101, 12, 102, 221], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(XMM6)), operand3: Some(Indirect(ESI, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 77, 15, 102, 14], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(EDI, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 117, 27, 102, 23], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM16)), operand3: Some(Direct(XMM10)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 125, 7, 102, 242], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(XMM10)), operand3: Some(Indirect(RAX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 45, 10, 102, 32], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K6)), operand2: Some(Direct(XMM30)), operand3: Some(IndirectScaledDisplaced(RDI, Two, 1780794939, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 13, 17, 102, 52, 125, 59, 198, 36, 106], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 125, 47, 102, 226], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM5)), operand3: Some(IndirectScaledIndexed(EDX, EDX, Two, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 85, 43, 102, 12, 82], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 113261947, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 93, 61, 102, 60, 85, 123, 61, 192, 6], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM31)), operand3: Some(Direct(YMM22)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 177, 5, 34, 102, 230], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(YMM28)), operand3: Some(IndirectScaledIndexed(RCX, RDI, Four, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 29, 36, 102, 12, 185], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K4)), operand2: Some(Direct(YMM15)), operand3: Some(IndirectDisplaced(RAX, 1219888941, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 5, 59, 102, 160, 45, 7, 182, 72], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM0)), operand3: Some(Direct(ZMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 125, 75, 102, 214], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledIndexedDisplaced(ESI, EDX, Eight, 1356546133, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 85, 74, 102, 156, 214, 85, 64, 219, 80], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K3)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledDisplaced(ECX, Eight, 690133709, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 101, 92, 102, 28, 205, 205, 154, 34, 41], OperandSize::Dword)
}

#[test]
fn vpcmpgtd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K7)), operand2: Some(Direct(ZMM16)), operand3: Some(Direct(ZMM15)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 209, 125, 71, 102, 255], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K1)), operand2: Some(Direct(ZMM28)), operand3: Some(Indirect(RDI, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 29, 71, 102, 15], OperandSize::Qword)
}

#[test]
fn vpcmpgtd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPCMPGTD, operand1: Some(Direct(K2)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexed(RDI, RAX, Two, Some(OperandSize::Dword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 53, 87, 102, 20, 71], OperandSize::Qword)
}

