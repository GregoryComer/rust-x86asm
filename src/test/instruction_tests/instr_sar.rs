use ::{BroadcastMode, Instruction, MaskReg, MergeMode, Mnemonic, OperandSize, Reg, RoundingMode};
use ::RegType::*;
use ::instruction_def::*;
use ::Operand::*;
use ::Reg::*;
use ::RegScale::*;
use ::test::run_test;

#[test]
fn sar_1() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 41], OperandSize::Word)
}

#[test]
fn sar_2() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(DI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(38)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 61, 38], OperandSize::Word)
}

#[test]
fn sar_3() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(33)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 33], OperandSize::Dword)
}

#[test]
fn sar_4() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(ESI, 14850871, Some(OperandSize::Byte), None)), operand2: Some(Literal8(110)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 190, 55, 155, 226, 0, 110], OperandSize::Dword)
}

#[test]
fn sar_5() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(31)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 250, 31], OperandSize::Qword)
}

#[test]
fn sar_6() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RBX, Eight, 1212489615, Some(OperandSize::Byte), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 60, 221, 143, 31, 69, 72, 41], OperandSize::Qword)
}

#[test]
fn sar_7() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Literal8(73)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 251, 73], OperandSize::Qword)
}

#[test]
fn sar_8() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RSI, Some(OperandSize::Byte), None)), operand2: Some(Literal8(46)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[192, 62, 46], OperandSize::Qword)
}

#[test]
fn sar_9() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(SP)), operand2: Some(Literal8(97)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 252, 97], OperandSize::Word)
}

#[test]
fn sar_10() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(SI, 2057, Some(OperandSize::Word), None)), operand2: Some(Literal8(104)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 188, 9, 8, 104], OperandSize::Word)
}

#[test]
fn sar_11() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(17)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 251, 17], OperandSize::Dword)
}

#[test]
fn sar_12() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EBX, Some(OperandSize::Word), None)), operand2: Some(Literal8(95)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 59, 95], OperandSize::Dword)
}

#[test]
fn sar_13() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DX)), operand2: Some(Literal8(113)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 250, 113], OperandSize::Qword)
}

#[test]
fn sar_14() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RBX, RAX, Four, 845260315, Some(OperandSize::Word), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 188, 131, 27, 166, 97, 50, 51], OperandSize::Qword)
}

#[test]
fn sar_15() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBP)), operand2: Some(Literal8(117)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 253, 117], OperandSize::Word)
}

#[test]
fn sar_16() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 15599, Some(OperandSize::Dword), None)), operand2: Some(Literal8(64)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 193, 185, 239, 60, 64], OperandSize::Word)
}

#[test]
fn sar_17() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(14)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 250, 14], OperandSize::Dword)
}

#[test]
fn sar_18() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EAX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(51)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 56, 51], OperandSize::Dword)
}

#[test]
fn sar_19() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(93)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 250, 93], OperandSize::Qword)
}

#[test]
fn sar_20() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDX, 483063443, Some(OperandSize::Dword), None)), operand2: Some(Literal8(126)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[193, 186, 147, 246, 202, 28, 126], OperandSize::Qword)
}

#[test]
fn sar_21() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RSI)), operand2: Some(Literal8(89)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 254, 89], OperandSize::Qword)
}

#[test]
fn sar_22() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RCX, 663664953, Some(OperandSize::Qword), None)), operand2: Some(Literal8(41)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 193, 185, 57, 185, 142, 39, 41], OperandSize::Qword)
}

#[test]
fn sar_23() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Word)
}

#[test]
fn sar_24() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 58], OperandSize::Word)
}

#[test]
fn sar_25() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Dword)
}

#[test]
fn sar_26() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(EBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 59], OperandSize::Dword)
}

#[test]
fn sar_27() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 250], OperandSize::Qword)
}

#[test]
fn sar_28() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RBX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 59], OperandSize::Qword)
}

#[test]
fn sar_29() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 249], OperandSize::Qword)
}

#[test]
fn sar_30() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RDX, Some(OperandSize::Byte), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[208, 58], OperandSize::Qword)
}

#[test]
fn sar_31() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 253], OperandSize::Word)
}

#[test]
fn sar_32() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(BX, DI, One, 101, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 121, 101], OperandSize::Word)
}

#[test]
fn sar_33() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 251], OperandSize::Dword)
}

#[test]
fn sar_34() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EDX, EBX, Eight, 255473738, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 188, 218, 74, 56, 58, 15], OperandSize::Dword)
}

#[test]
fn sar_35() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 255], OperandSize::Qword)
}

#[test]
fn sar_36() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RCX, 1936384213, Some(OperandSize::Word), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 185, 213, 224, 106, 115], OperandSize::Qword)
}

#[test]
fn sar_37() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EBX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 251], OperandSize::Word)
}

#[test]
fn sar_38() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(SI, 4703, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 209, 188, 95, 18], OperandSize::Word)
}

#[test]
fn sar_39() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESP)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 252], OperandSize::Dword)
}

#[test]
fn sar_40() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(ECX, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 57], OperandSize::Dword)
}

#[test]
fn sar_41() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 250], OperandSize::Qword)
}

#[test]
fn sar_42() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RAX, RSI, Eight, 989300441, Some(OperandSize::Dword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[209, 188, 240, 217, 134, 247, 58], OperandSize::Qword)
}

#[test]
fn sar_43() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RDX)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 250], OperandSize::Qword)
}

#[test]
fn sar_44() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(RBX, Some(OperandSize::Qword), None)), operand2: Some(Literal8(1)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 209, 59], OperandSize::Qword)
}

#[test]
fn sar_45() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Word)
}

#[test]
fn sar_46() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(BP, SI, One, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 58], OperandSize::Word)
}

#[test]
fn sar_47() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 250], OperandSize::Dword)
}

#[test]
fn sar_48() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(ECX, ECX, Two, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 60, 73], OperandSize::Dword)
}

#[test]
fn sar_49() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(BL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 251], OperandSize::Qword)
}

#[test]
fn sar_50() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledDisplaced(RBX, Four, 27087392, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 60, 157, 32, 82, 157, 1], OperandSize::Qword)
}

#[test]
fn sar_51() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CL)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 249], OperandSize::Qword)
}

#[test]
fn sar_52() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(RDX, 1849772140, Some(OperandSize::Byte), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[210, 186, 108, 72, 65, 110], OperandSize::Qword)
}

#[test]
fn sar_53() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 255], OperandSize::Word)
}

#[test]
fn sar_54() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Indirect(SI, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 60], OperandSize::Word)
}

#[test]
fn sar_55() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(CX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 249], OperandSize::Dword)
}

#[test]
fn sar_56() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(EBX, EDX, Two, 1529876727, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 188, 83, 247, 16, 48, 91], OperandSize::Dword)
}

#[test]
fn sar_57() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(DI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 255], OperandSize::Qword)
}

#[test]
fn sar_58() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RCX, RCX, Two, Some(OperandSize::Word), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 60, 73], OperandSize::Qword)
}

#[test]
fn sar_59() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 250], OperandSize::Word)
}

#[test]
fn sar_60() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectDisplaced(BX, 83, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[102, 211, 127, 83], OperandSize::Word)
}

#[test]
fn sar_61() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(EDX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 250], OperandSize::Dword)
}

#[test]
fn sar_62() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(EDX, ESI, Eight, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 60, 242], OperandSize::Dword)
}

#[test]
fn sar_63() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(ESI)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 254], OperandSize::Qword)
}

#[test]
fn sar_64() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexedDisplaced(RCX, RDI, Four, 836042308, Some(OperandSize::Dword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[211, 188, 185, 68, 254, 212, 49], OperandSize::Qword)
}

#[test]
fn sar_65() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(Direct(RCX)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 249], OperandSize::Qword)
}

#[test]
fn sar_66() {
    run_test(&Instruction { mnemonic: Mnemonic::SAR, operand1: Some(IndirectScaledIndexed(RSI, RDI, Four, Some(OperandSize::Qword), None)), operand2: Some(Direct(CL)), operand3: None, operand4: None, lock: false, rounding_mode: None, merge_mode: None, sae: false, mask: None, broadcast: None }, &[72, 211, 60, 190], OperandSize::Qword)
}

