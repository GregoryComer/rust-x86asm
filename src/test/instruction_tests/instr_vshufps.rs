use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(77)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 229, 77], OperandSize::Dword)
}

#[test]
fn vshufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: Some(IndirectScaledIndexed(ECX, EAX, Four, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(116)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 198, 52, 129, 116], OperandSize::Dword)
}

#[test]
fn vshufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM6)), operand4: Some(Literal8(28)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 198, 246, 28], OperandSize::Qword)
}

#[test]
fn vshufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 750742894, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(15)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 132, 65, 110, 109, 191, 44, 15], OperandSize::Qword)
}

#[test]
fn vshufps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(51)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 200, 51], OperandSize::Dword)
}

#[test]
fn vshufps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexed(EDI, EDI, Four, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(22)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 204, 198, 44, 191, 22], OperandSize::Dword)
}

#[test]
fn vshufps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM5)), operand4: Some(Literal8(25)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 245, 25], OperandSize::Qword)
}

#[test]
fn vshufps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RCX, 435715976, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(37)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 161, 136, 127, 248, 25, 37], OperandSize::Qword)
}

#[test]
fn vshufps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM2)), operand4: Some(Literal8(102)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 100, 139, 198, 226, 102], OperandSize::Dword)
}

#[test]
fn vshufps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(ESI, 385565484, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(123)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 84, 138, 198, 150, 44, 67, 251, 22, 123], OperandSize::Dword)
}

#[test]
fn vshufps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledIndexed(ESI, EAX, Eight, Some(OperandSize::Dword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 100, 153, 198, 36, 198, 43], OperandSize::Dword)
}

#[test]
fn vshufps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM28)), operand2: Some(Direct(XMM21)), operand3: Some(Direct(XMM16)), operand4: Some(Literal8(39)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 33, 84, 132, 198, 224, 39], OperandSize::Qword)
}

#[test]
fn vshufps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM11)), operand3: Some(Indirect(RDI, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 36, 142, 198, 23, 10], OperandSize::Qword)
}

#[test]
fn vshufps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM22)), operand2: Some(Direct(XMM15)), operand3: Some(IndirectScaledIndexed(RDX, RBX, Two, Some(OperandSize::Dword), None)), operand4: Some(Literal8(57)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 225, 4, 155, 198, 52, 90, 57], OperandSize::Qword)
}

#[test]
fn vshufps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 68, 170, 198, 216, 44], OperandSize::Dword)
}

#[test]
fn vshufps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM7)), operand3: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Two, 1120317399, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(121)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 68, 172, 198, 148, 64, 215, 175, 198, 66, 121], OperandSize::Dword)
}

#[test]
fn vshufps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(EBX, Some(OperandSize::Dword), None)), operand4: Some(Literal8(81)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 68, 186, 198, 43, 81], OperandSize::Dword)
}

#[test]
fn vshufps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM21)), operand3: Some(Direct(YMM3)), operand4: Some(Literal8(52)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 97, 84, 164, 198, 203, 52], OperandSize::Qword)
}

#[test]
fn vshufps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RDX, RCX, Four, 1155746814, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 225, 116, 174, 198, 132, 138, 254, 75, 227, 68, 89], OperandSize::Qword)
}

#[test]
fn vshufps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM9)), operand2: Some(Direct(YMM8)), operand3: Some(Indirect(RDI, Some(OperandSize::Dword), None)), operand4: Some(Literal8(26)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 60, 188, 198, 15, 26], OperandSize::Qword)
}

#[test]
fn vshufps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM3)), operand3: Some(Direct(ZMM3)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 205, 198, 195, 42], OperandSize::Dword)
}

#[test]
fn vshufps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectDisplaced(EDI, 1527349635, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(101)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 241, 100, 202, 198, 191, 131, 129, 9, 91, 101], OperandSize::Dword)
}

#[test]
fn vshufps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 1942203306, Some(OperandSize::Dword), None)), operand4: Some(Literal8(43)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 124, 220, 198, 28, 69, 170, 171, 195, 115, 43], OperandSize::Dword)
}

#[test]
fn vshufps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM12)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM7)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 113, 84, 203, 198, 231, 41], OperandSize::Qword)
}

#[test]
fn vshufps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM18)), operand2: Some(Direct(ZMM8)), operand3: Some(Indirect(RAX, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(115)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 225, 60, 204, 198, 16, 115], OperandSize::Qword)
}

#[test]
fn vshufps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM25)), operand2: Some(Direct(ZMM24)), operand3: Some(IndirectScaledDisplaced(RDI, Eight, 1746506049, Some(OperandSize::Dword), None)), operand4: Some(Literal8(106)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 60, 213, 198, 12, 253, 65, 145, 25, 104, 106], OperandSize::Qword)
}

