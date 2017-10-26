use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vshufps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(19)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 198, 228, 19], OperandSize::Dword)
}

#[test]
fn vshufps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EDI, ESI, Eight, 927083704, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(44)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 200, 198, 148, 247, 184, 44, 66, 55, 44], OperandSize::Dword)
}

#[test]
fn vshufps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM2)), operand3: Some(Direct(XMM5)), operand4: Some(Literal8(92)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 232, 198, 245, 92], OperandSize::Qword)
}

#[test]
fn vshufps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDX, Four, 1512003007, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(41)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 192, 198, 188, 147, 191, 85, 31, 90, 41], OperandSize::Qword)
}

#[test]
fn vshufps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(89)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 236, 89], OperandSize::Dword)
}

#[test]
fn vshufps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM7)), operand3: Some(Indirect(ECX, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(6)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 198, 33, 6], OperandSize::Dword)
}

#[test]
fn vshufps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM7)), operand4: Some(Literal8(9)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 196, 198, 223, 9], OperandSize::Qword)
}

#[test]
fn vshufps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM0)), operand3: Some(IndirectDisplaced(RSI, 2019214376, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(122)), lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 198, 190, 40, 196, 90, 120, 122], OperandSize::Qword)
}

#[test]
fn vshufps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: Some(Direct(XMM4)), operand4: Some(Literal8(40)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 100, 141, 198, 252, 40], OperandSize::Dword)
}

#[test]
fn vshufps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(EDI, Two, 2128743064, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(79)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 100, 137, 198, 4, 125, 152, 10, 226, 126, 79], OperandSize::Dword)
}

#[test]
fn vshufps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 760951737, Some(OperandSize::Dword), None)), operand4: Some(Literal8(66)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 241, 84, 158, 198, 60, 213, 185, 51, 91, 45, 66], OperandSize::Dword)
}

#[test]
fn vshufps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM15)), operand2: Some(Direct(XMM1)), operand3: Some(Direct(XMM17)), operand4: Some(Literal8(110)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 49, 116, 141, 198, 249, 110], OperandSize::Qword)
}

#[test]
fn vshufps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RCX, Some(OperandSize::Xmmword), None)), operand4: Some(Literal8(42)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 97, 116, 142, 198, 9, 42], OperandSize::Qword)
}

#[test]
fn vshufps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(XMM26)), operand2: Some(Direct(XMM19)), operand3: Some(IndirectScaledDisplaced(RCX, Two, 168484489, Some(OperandSize::Dword), None)), operand4: Some(Literal8(13)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 97, 100, 151, 198, 20, 77, 137, 222, 10, 10, 13], OperandSize::Qword)
}

#[test]
fn vshufps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: Some(Literal8(97)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 76, 172, 198, 200, 97], OperandSize::Dword)
}

#[test]
fn vshufps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(IndirectScaledIndexed(ECX, ECX, Eight, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(105)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 108, 171, 198, 44, 201, 105], OperandSize::Dword)
}

#[test]
fn vshufps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectDisplaced(EDX, 2091945465, Some(OperandSize::Dword), None)), operand4: Some(Literal8(72)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 241, 92, 186, 198, 178, 249, 141, 176, 124, 72], OperandSize::Dword)
}

#[test]
fn vshufps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM4)), operand4: Some(Literal8(47)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 68, 173, 198, 252, 47], OperandSize::Qword)
}

#[test]
fn vshufps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM20)), operand3: Some(IndirectScaledDisplaced(RAX, Eight, 1332357419, Some(OperandSize::Ymmword), None)), operand4: Some(Literal8(71)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 92, 167, 198, 12, 197, 43, 41, 106, 79, 71], OperandSize::Qword)
}

#[test]
fn vshufps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RAX, Two, 1959100405, Some(OperandSize::Dword), None)), operand4: Some(Literal8(34)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 113, 116, 189, 198, 148, 65, 245, 127, 197, 116, 34], OperandSize::Qword)
}

#[test]
fn vshufps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM1)), operand3: Some(Direct(ZMM6)), operand4: Some(Literal8(50)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 116, 206, 198, 230, 50], OperandSize::Dword)
}

#[test]
fn vshufps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM2)), operand3: Some(IndirectScaledDisplaced(ESI, Eight, 440613829, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 108, 205, 198, 36, 245, 197, 59, 67, 26, 10], OperandSize::Dword)
}

#[test]
fn vshufps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM5)), operand2: Some(Direct(ZMM7)), operand3: Some(IndirectScaledDisplaced(EBX, Eight, 2085113502, Some(OperandSize::Dword), None)), operand4: Some(Literal8(54)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 241, 68, 221, 198, 44, 221, 158, 78, 72, 124, 54], OperandSize::Dword)
}

#[test]
fn vshufps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM14)), operand2: Some(Direct(ZMM14)), operand3: Some(Direct(ZMM25)), operand4: Some(Literal8(114)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 17, 12, 204, 198, 241, 114], OperandSize::Qword)
}

#[test]
fn vshufps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM8)), operand2: Some(Direct(ZMM25)), operand3: Some(IndirectScaledIndexedDisplaced(RBX, RDI, Two, 1113567867, Some(OperandSize::Zmmword), None)), operand4: Some(Literal8(70)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 113, 52, 196, 198, 132, 123, 123, 178, 95, 66, 70], OperandSize::Qword)
}

#[test]
fn vshufps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VSHUFPS, operand1: Some(Direct(ZMM24)), operand2: Some(Direct(ZMM3)), operand3: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Two, 1551295740, Some(OperandSize::Dword), None)), operand4: Some(Literal8(10)), lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To16) }, &[98, 97, 100, 221, 198, 132, 112, 252, 228, 118, 92, 10], OperandSize::Qword)
}

