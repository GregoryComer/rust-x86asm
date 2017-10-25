use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;

fn vmovaps_1() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 241], OperandSize::Dword)
}

fn vmovaps_2() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM0)), operand2: Some(IndirectScaledIndexedDisplaced(EDX, EAX, Eight, 128745910, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 132, 194, 182, 129, 172, 7], OperandSize::Dword)
}

fn vmovaps_3() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 234], OperandSize::Qword)
}

fn vmovaps_4() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM2)), operand2: Some(IndirectScaledIndexedDisplaced(RSI, RAX, Eight, 175271139, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 148, 198, 227, 108, 114, 10], OperandSize::Qword)
}

fn vmovaps_5() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 230], OperandSize::Dword)
}

fn vmovaps_6() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(EDX, 1175278119, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 170, 39, 82, 13, 70], OperandSize::Dword)
}

fn vmovaps_7() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 250], OperandSize::Qword)
}

fn vmovaps_8() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM0)), operand2: Some(IndirectScaledDisplaced(RSI, Four, 827121938, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 4, 181, 18, 225, 76, 49], OperandSize::Qword)
}

fn vmovaps_9() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 241, 124, 143, 40, 237], OperandSize::Dword)
}

fn vmovaps_10() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM4)), operand2: Some(Indirect(EAX, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 140, 40, 32], OperandSize::Dword)
}

fn vmovaps_11() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM17)), operand2: Some(Direct(XMM20)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K2), broadcast: None }, &[98, 161, 124, 138, 40, 204], OperandSize::Qword)
}

fn vmovaps_12() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM26)), operand2: Some(IndirectScaledDisplaced(RDI, Two, 1360481424, Some(OperandSize::Xmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 97, 124, 143, 40, 20, 125, 144, 76, 23, 81], OperandSize::Qword)
}

fn vmovaps_13() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM6)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 174, 40, 245], OperandSize::Dword)
}

fn vmovaps_14() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM5)), operand2: Some(IndirectDisplaced(ECX, 1535200657, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 40, 169, 145, 77, 129, 91], OperandSize::Dword)
}

fn vmovaps_15() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM8)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 65, 124, 169, 40, 200], OperandSize::Qword)
}

fn vmovaps_16() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM23)), operand2: Some(IndirectScaledDisplaced(RDX, Eight, 1661476856, Some(OperandSize::Ymmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 225, 124, 171, 40, 60, 213, 248, 31, 8, 99], OperandSize::Qword)
}

fn vmovaps_17() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM1)), operand2: Some(Direct(ZMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 206, 40, 205], OperandSize::Dword)
}

fn vmovaps_18() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM6)), operand2: Some(Indirect(EDX, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 241, 124, 203, 40, 50], OperandSize::Dword)
}

fn vmovaps_19() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM23)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K7), broadcast: None }, &[98, 33, 124, 207, 40, 239], OperandSize::Qword)
}

fn vmovaps_20() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM5)), operand2: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Two, 748169720, Some(OperandSize::Zmmword), None)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 241, 124, 204, 40, 172, 121, 248, 41, 152, 44], OperandSize::Qword)
}

fn vmovaps_21() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM5)), operand2: Some(Direct(XMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 236], OperandSize::Dword)
}

fn vmovaps_22() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Two, 1669293893, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 172, 114, 69, 103, 127, 99], OperandSize::Dword)
}

fn vmovaps_23() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM6)), operand2: Some(Direct(XMM0)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 40, 240], OperandSize::Qword)
}

fn vmovaps_24() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(RBX, 1624481310, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 187, 30, 158, 211, 96], OperandSize::Qword)
}

fn vmovaps_25() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM7)), operand2: Some(Direct(YMM2)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 250], OperandSize::Dword)
}

fn vmovaps_26() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectDisplaced(EDX, 1298620568, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 162, 152, 96, 103, 77], OperandSize::Dword)
}

fn vmovaps_27() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM2)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 40, 209], OperandSize::Qword)
}

fn vmovaps_28() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RAX, Two, 1572452188, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 60, 69, 92, 183, 185, 93], OperandSize::Qword)
}

fn vmovaps_29() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM7)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K6), broadcast: None }, &[98, 241, 124, 142, 40, 251], OperandSize::Dword)
}

fn vmovaps_30() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexedDisplaced(EAX, EAX, Four, 668284746, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM3)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 156, 128, 74, 55, 213, 39], OperandSize::Dword)
}

fn vmovaps_31() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(XMM29)), operand2: Some(Direct(XMM12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K4), broadcast: None }, &[98, 65, 124, 140, 40, 236], OperandSize::Qword)
}

fn vmovaps_32() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(RDX, RCX, Two, Some(OperandSize::Xmmword), None)), operand2: Some(Direct(XMM6)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 248, 41, 52, 74], OperandSize::Qword)
}

fn vmovaps_33() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM4)), operand2: Some(Direct(YMM5)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 241, 124, 173, 40, 229], OperandSize::Dword)
}

fn vmovaps_34() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledIndexed(EAX, EDI, Two, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 252, 41, 12, 120], OperandSize::Dword)
}

fn vmovaps_35() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(YMM25)), operand2: Some(Direct(YMM24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K3), broadcast: None }, &[98, 1, 124, 171, 40, 200], OperandSize::Qword)
}

fn vmovaps_36() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Indirect(RSI, Some(OperandSize::Ymmword), None)), operand2: Some(Direct(YMM13)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[197, 124, 41, 46], OperandSize::Qword)
}

fn vmovaps_37() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM7)), operand2: Some(Direct(ZMM7)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K1), broadcast: None }, &[98, 241, 124, 201, 40, 255], OperandSize::Dword)
}

fn vmovaps_38() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(EDI, Eight, 748762116, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM4)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 36, 253, 4, 52, 161, 44], OperandSize::Dword)
}

fn vmovaps_39() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(Direct(ZMM29)), operand2: Some(Direct(ZMM25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: Some(MergeMode::Zero), sae: false, mask: Some(MaskReg::K5), broadcast: None }, &[98, 1, 124, 205, 40, 233], OperandSize::Qword)
}

fn vmovaps_40() {
    run_test(&Instruction { mnemonic: Mnemonic::VMOVAPS, operand1: Some(IndirectScaledDisplaced(RDI, Two, 2037326685, Some(OperandSize::Zmmword), None)), operand2: Some(Direct(ZMM1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[98, 241, 124, 72, 41, 12, 125, 93, 35, 111, 121], OperandSize::Qword)
}

