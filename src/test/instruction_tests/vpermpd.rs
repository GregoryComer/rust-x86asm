use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vpermpd_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Direct(YMM6)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 242, 213, 175, 22, 238], OperandSize::Dword)
}

fn vpermpd_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM6)), operand3: Some(IndirectScaledIndexedDisplaced(EBX, EDI, Four, 1264558157, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 242, 205, 171, 22, 140, 187, 77, 160, 95, 75], OperandSize::Dword)
}

fn vpermpd_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM2)), operand3: Some(Indirect(EAX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 242, 237, 188, 22, 40], OperandSize::Dword)
}

fn vpermpd_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM0)), operand3: Some(Direct(YMM8)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 210, 253, 175, 22, 240], OperandSize::Qword)
}

fn vpermpd_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Direct(YMM27)), operand3: Some(IndirectScaledIndexedDisplaced(RCX, RDX, Eight, 3271209, Some(OperandSize::Ymmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 242, 165, 166, 22, 132, 209, 41, 234, 49, 0], OperandSize::Qword)
}

fn vpermpd_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM31)), operand2: Some(Direct(YMM16)), operand3: Some(IndirectDisplaced(RAX, 1163132856, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 98, 253, 179, 22, 184, 184, 255, 83, 69], OperandSize::Qword)
}

fn vpermpd_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM0)), operand2: Some(Direct(ZMM7)), operand3: Some(Direct(ZMM0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 242, 197, 201, 22, 192], OperandSize::Dword)
}

fn vpermpd_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(IndirectDisplaced(EAX, 1267567370, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 242, 221, 204, 22, 152, 10, 139, 141, 75], OperandSize::Dword)
}

fn vpermpd_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM1)), operand3: Some(IndirectScaledIndexedDisplaced(EDX, ECX, Two, 1129554849, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 242, 245, 219, 22, 140, 74, 161, 163, 83, 67], OperandSize::Dword)
}

fn vpermpd_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(Direct(ZMM6)), operand3: Some(Direct(ZMM11)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 210, 205, 206, 22, 211], OperandSize::Qword)
}

fn vpermpd_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM18)), operand3: Some(Indirect(RCX, Some(OperandSize::Zmmword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 226, 237, 196, 22, 49], OperandSize::Qword)
}

fn vpermpd_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM17)), operand2: Some(Direct(ZMM10)), operand3: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 226, 173, 218, 22, 9], OperandSize::Qword)
}

fn vpermpd_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM3)), operand2: Some(Direct(YMM7)), operand3: Some(Literal8(88)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 223, 88], OperandSize::Dword)
}

fn vpermpd_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(IndirectScaledDisplaced(EAX, Four, 375657338, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(107)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 12, 133, 122, 19, 100, 22, 107], OperandSize::Dword)
}

fn vpermpd_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM1)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(82)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 205, 82], OperandSize::Qword)
}

fn vpermpd_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM0)), operand2: Some(Indirect(RDI, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(28)), operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[196, 227, 253, 1, 7, 28], OperandSize::Qword)
}

fn vpermpd_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(Direct(YMM5)), operand3: Some(Literal8(40)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 243, 253, 175, 1, 237, 40], OperandSize::Dword)
}

fn vpermpd_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM7)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1712307141, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(78)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 174, 1, 188, 114, 197, 187, 15, 102, 78], OperandSize::Dword)
}

fn vpermpd_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM5)), operand2: Some(IndirectScaledDisplaced(ESI, Four, 88842210, Some(OperandSize::Qword), None)), operand3: Some(Literal8(41)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 243, 253, 186, 1, 44, 181, 226, 159, 75, 5, 41], OperandSize::Dword)
}

fn vpermpd_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM11)), operand2: Some(Direct(YMM17)), operand3: Some(Literal8(121)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 51, 253, 175, 1, 217, 121], OperandSize::Qword)
}

fn vpermpd_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM19)), operand2: Some(IndirectScaledIndexed(RSI, RSI, Four, Some(OperandSize::Ymmword), None)), operand3: Some(Literal8(83)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 227, 253, 172, 1, 28, 182, 83], OperandSize::Qword)
}

fn vpermpd_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(YMM13)), operand2: Some(Indirect(RCX, Some(OperandSize::Qword), None)), operand3: Some(Literal8(0)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: Some(BroadcastMode::Broadcast1To4) }, &[98, 115, 253, 186, 1, 41, 0], OperandSize::Qword)
}

fn vpermpd_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM3)), operand2: Some(Direct(ZMM4)), operand3: Some(Literal8(16)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 243, 253, 203, 1, 220, 16], OperandSize::Dword)
}

fn vpermpd_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM7)), operand2: Some(IndirectScaledIndexed(EBX, EAX, Four, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(104)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 243, 253, 206, 1, 60, 131, 104], OperandSize::Dword)
}

fn vpermpd_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM2)), operand2: Some(IndirectScaledIndexedDisplaced(ESI, ECX, Four, 691137464, Some(OperandSize::Qword), None)), operand3: Some(Literal8(59)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 243, 253, 220, 1, 148, 142, 184, 235, 49, 41, 59], OperandSize::Dword)
}

fn vpermpd_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM22)), operand2: Some(Direct(ZMM5)), operand3: Some(Literal8(37)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 227, 253, 201, 1, 245, 37], OperandSize::Qword)
}

fn vpermpd_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM12)), operand2: Some(Indirect(RSI, Some(OperandSize::Zmmword), None)), operand3: Some(Literal8(111)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 115, 253, 202, 1, 38, 111], OperandSize::Qword)
}

fn vpermpd_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VPERMPD, operand1: Some(Direct(ZMM15)), operand2: Some(IndirectScaledIndexed(RCX, RDI, Two, Some(OperandSize::Qword), None)), operand3: Some(Literal8(98)), operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: Some(BroadcastMode::Broadcast1To8) }, &[98, 115, 253, 222, 1, 60, 121, 98], OperandSize::Qword)
}

