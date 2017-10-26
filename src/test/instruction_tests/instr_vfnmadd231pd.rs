use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn vfnmadd231pd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM6)), operand3: Some(Direct(XMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 201, 188, 206], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM1)), operand2: Some(Direct(XMM5)), operand3: Some(IndirectDisplaced(EDI, 1360159752, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 209, 188, 143, 8, 100, 18, 81], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM3)), operand2: Some(Direct(XMM7)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 193, 188, 218], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM4)), operand2: Some(Direct(XMM1)), operand3: Some(Indirect(RBX, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 241, 188, 35], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM6)), operand3: Some(Direct(YMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 205, 188, 248], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM4)), operand3: Some(IndirectScaledDisplaced(EDI, Eight, 1438832550, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 221, 188, 36, 253, 166, 215, 194, 85], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 253, 188, 222], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM1)), operand3: Some(IndirectScaledDisplaced(RSI, Eight, 994681282, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 226, 245, 188, 28, 245, 194, 161, 73, 59], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM2)), operand2: Some(Direct(XMM4)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 140, 188, 210], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM7)), operand3: Some(IndirectScaledDisplaced(EAX, Two, 397231163, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 197, 143, 188, 60, 69, 59, 68, 173, 23], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM0)), operand2: Some(Direct(XMM4)), operand3: Some(Indirect(EDX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 242, 221, 153, 188, 2], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM10)), operand2: Some(Direct(XMM17)), operand3: Some(Direct(XMM2)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 114, 245, 129, 188, 210], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM3)), operand3: Some(IndirectScaledDisplaced(RBX, Four, 1920914327, Some(OperandSize::Xmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 242, 229, 138, 188, 52, 157, 151, 211, 126, 114], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(XMM25)), operand2: Some(Direct(XMM0)), operand3: Some(Indirect(RSI, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To2) }, &[98, 98, 253, 154, 188, 14], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM7)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 169, 188, 206], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(EDX, Eight, 812493944, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 188, 12, 213, 120, 172, 109, 48], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledDisplaced(ESI, Two, 2059833664, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 205, 186, 188, 52, 117, 64, 145, 198, 122], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM16)), operand2: Some(Direct(YMM1)), operand3: Some(Direct(YMM7)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 226, 245, 170, 188, 199], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM28)), operand3: Some(Indirect(RDX, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 157, 161, 188, 26], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(YMM10)), operand2: Some(Direct(YMM17)), operand3: Some(IndirectScaledDisplaced(RDX, Eight, 1274215755, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 114, 245, 181, 188, 20, 213, 75, 253, 242, 75], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM4)), operand2: Some(Direct(ZMM5)), operand3: Some(Direct(ZMM3)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 213, 158, 188, 227], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM0)), operand3: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 253, 207, 188, 58], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM0)), operand3: Some(IndirectScaledDisplaced(EDX, Two, 863182613, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 253, 222, 188, 12, 85, 21, 31, 115, 51], OperandSize::Dword)
}

#[test]
fn vfnmadd231pd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM15)), operand2: Some(Direct(ZMM17)), operand3: Some(Direct(ZMM23)), operand4: None, lock: false, rounding_mode: Some(RoundingMode::Nearest), merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 50, 245, 150, 188, 255], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM19)), operand3: Some(IndirectScaledDisplaced(RSI, Two, 95933221, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 229, 196, 188, 12, 117, 37, 211, 183, 5], OperandSize::Qword)
}

#[test]
fn vfnmadd231pd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VFNMADD231PD, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM5)), operand3: Some(IndirectScaledDisplaced(RSI, Four, 474727993, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 98, 213, 219, 188, 44, 181, 57, 198, 75, 28], OperandSize::Qword)
}

