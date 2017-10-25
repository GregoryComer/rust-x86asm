use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(34)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 34], OperandSize::Word)
}

#[test]
fn sar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(68)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 56, 68], OperandSize::Word)
}

#[test]
fn sar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(52)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 52], OperandSize::Dword)
}

#[test]
fn sar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, ESI, Eight, 1977755277, Some(OperandSize::Byte), None)), operand2: Some(Literal8(29)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 188, 242, 141, 38, 226, 117, 29], OperandSize::Dword)
}

#[test]
fn sar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 113], OperandSize::Qword)
}

#[test]
fn sar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RBX, 636548322, Some(OperandSize::Byte), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 187, 226, 244, 240, 37, 125], OperandSize::Qword)
}

#[test]
fn sar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 113], OperandSize::Qword)
}

#[test]
fn sar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RAX, 2072010646, Some(OperandSize::Byte), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 184, 150, 95, 128, 123, 104], OperandSize::Qword)
}

#[test]
fn sar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(12)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 251, 12], OperandSize::Word)
}

#[test]
fn sar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 222, Some(OperandSize::Word), None)), operand2: Some(Literal8(24)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 184, 222, 0, 24], OperandSize::Word)
}

#[test]
fn sar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Literal8(42)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 254, 42], OperandSize::Dword)
}

#[test]
fn sar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(ESI, 1117502858, Some(OperandSize::Word), None)), operand2: Some(Literal8(45)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 190, 138, 189, 155, 66, 45], OperandSize::Dword)
}

#[test]
fn sar_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Literal8(123)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 249, 123], OperandSize::Qword)
}

#[test]
fn sar_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RAX, Some(OperandSize::Word), None)), operand2: Some(Literal8(81)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 56, 81], OperandSize::Qword)
}

#[test]
fn sar_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 252, 117], OperandSize::Word)
}

#[test]
fn sar_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, SI, One, 25188, Some(OperandSize::Dword), None)), operand2: Some(Literal8(125)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 184, 100, 98, 125], OperandSize::Word)
}

#[test]
fn sar_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 249, 95], OperandSize::Dword)
}

#[test]
fn sar_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(87)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 58, 87], OperandSize::Dword)
}

#[test]
fn sar_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(94)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 251, 94], OperandSize::Qword)
}

#[test]
fn sar_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDI, 816963762, Some(OperandSize::Dword), None)), operand2: Some(Literal8(30)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 191, 178, 224, 177, 48, 30], OperandSize::Qword)
}

#[test]
fn sar_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(25)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 250, 25], OperandSize::Qword)
}

#[test]
fn sar_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RDI, Two, 480826952, Some(OperandSize::Qword), None)), operand2: Some(Literal8(120)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 60, 125, 72, 214, 168, 28, 120], OperandSize::Qword)
}

#[test]
fn sar_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Word)
}

#[test]
fn sar_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BP, DI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 59], OperandSize::Word)
}

#[test]
fn sar_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 251], OperandSize::Dword)
}

#[test]
fn sar_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(EBX, 106838295, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 187, 23, 57, 94, 6], OperandSize::Dword)
}

#[test]
fn sar_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 251], OperandSize::Qword)
}

#[test]
fn sar_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RDX, RDX, Eight, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 60, 210], OperandSize::Qword)
}

#[test]
fn sar_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Qword)
}

#[test]
fn sar_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 59], OperandSize::Qword)
}

#[test]
fn sar_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 254], OperandSize::Word)
}

#[test]
fn sar_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 56], OperandSize::Word)
}

#[test]
fn sar_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 255], OperandSize::Dword)
}

#[test]
fn sar_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EAX, Two, 1055152016, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 188, 67, 144, 87, 228, 62], OperandSize::Dword)
}

#[test]
fn sar_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 253], OperandSize::Qword)
}

#[test]
fn sar_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1948773519, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 60, 221, 143, 236, 39, 116], OperandSize::Qword)
}

#[test]
fn sar_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 253], OperandSize::Word)
}

#[test]
fn sar_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Memory(6861, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 62, 205, 26], OperandSize::Word)
}

#[test]
fn sar_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ECX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 249], OperandSize::Dword)
}

#[test]
fn sar_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EDX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 58], OperandSize::Dword)
}

#[test]
fn sar_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 254], OperandSize::Qword)
}

#[test]
fn sar_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RCX, 911108277, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 185, 181, 104, 78, 54], OperandSize::Qword)
}

#[test]
fn sar_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 250], OperandSize::Qword)
}

#[test]
fn sar_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RSI, RAX, Eight, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 60, 198], OperandSize::Qword)
}

#[test]
fn sar_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 249], OperandSize::Word)
}

#[test]
fn sar_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BP, SI, One, 23396, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 186, 100, 91], OperandSize::Word)
}

#[test]
fn sar_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 251], OperandSize::Dword)
}

#[test]
fn sar_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(ESI, 1803339873, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 190, 97, 200, 124, 107], OperandSize::Dword)
}

#[test]
fn sar_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Qword)
}

#[test]
fn sar_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RDX, RDI, Eight, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 60, 250], OperandSize::Qword)
}

#[test]
fn sar_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Qword)
}

#[test]
fn sar_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RSI, Eight, 2099194507, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 188, 241, 139, 42, 31, 125], OperandSize::Qword)
}

#[test]
fn sar_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 254], OperandSize::Word)
}

#[test]
fn sar_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(BX, 4343, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 191, 247, 16], OperandSize::Word)
}

#[test]
fn sar_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 250], OperandSize::Dword)
}

#[test]
fn sar_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, ECX, Four, 870064549, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 188, 139, 165, 33, 220, 51], OperandSize::Dword)
}

#[test]
fn sar_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 252], OperandSize::Qword)
}

#[test]
fn sar_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RDI, RCX, Four, 1926486474, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 188, 143, 202, 217, 211, 114], OperandSize::Qword)
}

#[test]
fn sar_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 252], OperandSize::Word)
}

#[test]
fn sar_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BX, SI, One, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 56], OperandSize::Word)
}

#[test]
fn sar_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 250], OperandSize::Dword)
}

#[test]
fn sar_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 57], OperandSize::Dword)
}

#[test]
fn sar_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 252], OperandSize::Qword)
}

#[test]
fn sar_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RCX, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 57], OperandSize::Qword)
}

#[test]
fn sar_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RSI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 254], OperandSize::Qword)
}

#[test]
fn sar_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDI, 643486550, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 191, 86, 211, 90, 38], OperandSize::Qword)
}

